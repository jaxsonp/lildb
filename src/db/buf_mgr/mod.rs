mod pageref;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::{Mutex, Weak};
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use rustc_hash::FxBuildHasher;

use crate::*;
use db::*;
use disk_mgr::DiskManager;
use pageref::PageRef;

/// How often to check for a free buffer slot when pool is full, in ms
const POLL_SLEEP_TIME: u64 = 100;

lazy_static! {
	/// THE global buffer manager, access through `buf_mgr!()`
	pub(crate) static ref GLOBAL_BUF_MGR: Arc<Mutex<BufferManager>> = Arc::new(Mutex::new(BufferManager::new()));
}

/// Convenience macro to lock and gain access to the global buffer manager
macro_rules! buf_mgr {
	() => {
		match crate::db::buf_mgr::GLOBAL_BUF_MGR.lock() {
			Ok(lock) => Ok(lock),
			Err(e) => Err(Error::wrap(
				Internal,
				"Error while accessing global buffer manager",
				e,
			)),
		}
	};
}

/// Buffer pool slot, contains info about
struct BufPoolSlot {
	/// Database this slot belongs to
	db_id: DatabaseId,
	/// Which page number is in this slot
	page_id: PageId,
	/// The shared ptr to the contained page
	page_lock: Arc<RwLock<Page>>,
	/// Number of concurrent holders of this page, when this value is 0 the slot is not in use and
	/// can be evicted to make room for another page
	pin_count: u32,
	/// Last time this slot was pinned, used for LRU eviction
	last_access: u128,
	/// Whether this page has been written to
	dirty_lock: Arc<Mutex<bool>>,
}

/// Manages a buffer pool, serving pages while loading, caching, and flushing pages to memory when needed
///
/// Intended to be constructed as a global static `Arc<Mutex<BufferManager>>`, then accessed
/// through the `buf_mgr!()` macro
pub struct BufferManager {
	/// Buffer pool, starts with length 0 then grows until full
	buf_pool: Vec<BufPoolSlot>,
	/// Tracks every page in the pool and its index
	page_index: HashMap<(DatabaseId, PageId), usize, FxBuildHasher>,
	/// Ticks up every slot access, used by each slot to track order of accesses
	access_count: u128,
	/// Remembers disk managers for when its time to flush pages
	dm_registry: HashMap<DatabaseId, Weak<Mutex<DiskManager>>, FxBuildHasher>,
}
impl BufferManager {
	/// Size of buffer pool (max number of pages simulataneously in memory)
	const POOL_SIZE: usize = if cfg!(test) { 10 } else { 100 };

	pub(self) fn new() -> Self {
		log::info!(
			"Instantiating buffer manager with pool size {}",
			BufferManager::POOL_SIZE
		);
		BufferManager {
			buf_pool: Vec::with_capacity(BufferManager::POOL_SIZE),
			page_index: HashMap::with_hasher(FxBuildHasher),
			access_count: 0,
			dm_registry: HashMap::with_hasher(FxBuildHasher),
		}
	}

	/// Pins a page in the global buffer pool, effectively marking it as "in use"
	///
	/// If the page is already in the buffer pool, this function will just return that. If not, it
	/// will choose the least recently used (LRU) slot, and "evict" it, and load the page from disk
	/// into that slot
	///
	/// The returned `PageRef` will unpin itself when dropped
	pub fn pin(page_id: PageId, dm: &Arc<Mutex<DiskManager>>) -> Result<PageRef, Error> {
		log::debug!("Pinning page {page_id} from db \"{}\"", dm.lock()?.db_name,);

		let mut bm = buf_mgr!()?;
		bm.access_count += 1;
		let access_count = bm.access_count;
		let db_id = dm.lock()?.db_id;
		bm.dm_registry.insert(db_id, Arc::downgrade(dm));

		if let Some(index) = bm.page_index.get(&(db_id, page_id)) {
			let index = *index;

			// page is already in pool
			let slot = &mut bm.buf_pool[index];
			log::trace!(
				"Found page in buffer pool at index {index} with pin count {}",
				slot.pin_count
			);

			slot.pin_count += 1;
			slot.last_access = access_count;

			Ok(PageRef {
				page_lock: slot.page_lock.clone(),
				dirty_lock: slot.dirty_lock.clone(),
				index,
				db_id,
				page_id,
			})
		} else {
			// need to read page from disk
			let raw_page = match dm.lock()?.read_page(page_id) {
				Ok(p) => p,
				Err(e) => {
					return Err(Error::wrap(Internal, "Error while pinning page", e));
				}
			};

			if bm.buf_pool.len() < BufferManager::POOL_SIZE {
				// pool isn't full yet
				let index = bm.buf_pool.len();
				bm.buf_pool.push(BufPoolSlot {
					db_id,
					page_id,
					page_lock: Arc::new(RwLock::new(raw_page)),
					pin_count: 0,
					last_access: access_count,
					dirty_lock: Arc::new(Mutex::new(false)),
				});
				log::trace!(
					"Expanding buffer pool, length {}/{}",
					bm.buf_pool.len(),
					BufferManager::POOL_SIZE
				);

				bm.page_index.insert((db_id, page_id), index);
				let slot = &mut bm.buf_pool[index];
				slot.pin_count += 1;

				Ok(PageRef {
					page_lock: slot.page_lock.clone(),
					dirty_lock: slot.dirty_lock.clone(),
					index,
					db_id,
					page_id,
				})
			} else {
				// pool is full, choosing a slot to replace

				// don't want to hold the lock on the buffer manager while polling for an empty
				// slot
				drop(bm);

				let mut i = 0;
				loop {
					// choosing which slot to evict
					let mut choice: Option<usize> = None;
					let mut earliest = u128::MAX;
					let mut bm = buf_mgr!()?;
					for (i, slot) in bm.buf_pool.iter().enumerate() {
						if slot.pin_count == 0 && slot.last_access < earliest {
							earliest = slot.last_access;
							choice = Some(i);
						}
					}
					if let Some(index) = choice {
						// found one

						log::trace!(
							"Evicting LRU page at index {index} (pin count: {}) ({} ms)",
							bm.buf_pool[index].pin_count,
							i * POLL_SLEEP_TIME
						);

						// evicting
						bm.flush(index)?;
						let old_key = (bm.buf_pool[index].db_id, bm.buf_pool[index].page_id);
						bm.page_index.remove(&old_key);

						// overwriting
						bm.buf_pool[index] = BufPoolSlot {
							db_id,
							page_id,
							page_lock: Arc::new(RwLock::new(raw_page)),
							pin_count: 0,
							last_access: bm.access_count,
							dirty_lock: Arc::new(Mutex::new(false)),
						};

						log::trace!("Inserting page into buffer pool at index {index}");
						bm.page_index.insert((db_id, page_id), index);
						let slot = &mut bm.buf_pool[index];
						slot.pin_count += 1;

						break Ok(PageRef {
							page_lock: slot.page_lock.clone(),
							dirty_lock: slot.dirty_lock.clone(),
							index,
							db_id,
							page_id,
						});
					}

					// did not find unpinned slot, try again later

					// Let go of global buffer manager lock while polling
					drop(bm);
					sleep(Duration::from_millis(POLL_SLEEP_TIME));
					i += 1;
				}
			}
		}
	}

	/// Unpins a page, opposite to `pin()`.
	///
	/// Should only be called by the PageRef destructor
	fn unpin(index: usize) -> Result<(), Error> {
		let mut bm = buf_mgr!()?;
		let slot = &mut bm.buf_pool[index];
		slot.pin_count = slot.pin_count.saturating_sub(1);
		log::debug!(
			"Unpinned page at index {} (pin count now {})",
			index,
			slot.pin_count
		);
		Ok(())
	}

	/// Flushes a page to disk if it has been written to (aka if it's "dirty")
	///
	/// Uses `BufferManager`'s `dm_registry` to try and "remember" the disk manager, if the disk
	/// manager has since been dropped, reopen another one temporarily to write page
	fn flush(&mut self, index: usize) -> Result<(), Error> {
		let slot = &mut self.buf_pool[index];
		let mut dirty = slot.dirty_lock.lock()?;
		if *dirty {
			let page = slot.page_lock.read()?;

			if let Some(dm_ref) = self.dm_registry.get(&slot.db_id) {
				// disk manager is in registry

				if let Some(dm) = dm_ref.upgrade() {
					// disk manager hasn't been dropped yet
					dm.lock()?.write_page(&page)?;
				}
			}

			*dirty = false;
		}
		Ok(())
	}
}

impl Drop for BufferManager {
	fn drop(&mut self) {
		// flushing all pages on drop
		log::info!("Flushing all pages");
		for i in 0..self.buf_pool.len() {
			let _ = self.flush(i);
		}
	}
}
