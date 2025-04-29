mod pageref;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::RwLockWriteGuard;
use std::thread::sleep;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use rustc_hash::FxBuildHasher;

use crate::*;
use db::*;
use disk_mgr::DiskManager;
use pageref::PageRef;

/// How often to check for a free buffer slot when pool is full, in ms
const POLL_SLEEP_TIME: u64 = 50;

lazy_static! {
	/// THE global buffer manager
	static ref GLOBAL_BUF_MGR: Arc<RwLock<BufferManager>> = Arc::new(RwLock::new(BufferManager::new(BUF_POOL_SIZE)));
	/// Used as a sort of `Instant::MIN` for instant comparisons
	static ref INIT_TIME: Instant = Instant::now();
}

struct BufPoolSlot {
	page_id: PageId,
	page_lock: Arc<RwLock<Page>>,
	/// Number of concurrent holders of this page
	pin_count: u32,
	/// Last time this slot was pinned, used for LRU eviction
	last_access: Instant,
	/// Whether this page has been written to
	dirty: bool,
}

/// Manages a buffer pool, serving pages while loading and flushing pages to memory when needed
///
/// Intended to be constructed as a global static `Arc<RwLock<BufferManager>>`, then accessed
/// through the `BufferManager::access()` static method
pub struct BufferManager {
	/// Maximum number of pages that can be loaded at once
	pool_size: usize,
	/// Buffer pool, starts with length 0 then grows until full
	buf_pool: Vec<BufPoolSlot>,
	/// Tracks every page in the pool and its index
	page_index: HashMap<PageId, usize, FxBuildHasher>,
}
impl BufferManager {
	pub(self) fn new(pool_size: usize) -> Self {
		BufferManager {
			pool_size,
			buf_pool: Vec::with_capacity(pool_size),
			page_index: HashMap::with_hasher(FxBuildHasher),
		}
	}

	/// Convenience function to obtain a mut lock on the global buffer manager
	pub fn access() -> Result<RwLockWriteGuard<'static, BufferManager>, Error> {
		match GLOBAL_BUF_MGR.write() {
			Ok(lock) => Ok(lock),
			Err(e) => Err(Error::wrap(
				InternalError,
				"Error while accessing global buffer manager",
				e,
			)),
		}
	}

	/// Pins a page in the buffer pool, effectively marking it as "in use"
	pub fn pin(&mut self, page_id: PageId, dm: &mut DiskManager) -> Result<PageRef, Error> {
		if let Some(index) = self.page_index.get(&page_id) {
			// page is in pool
			let slot = &mut self.buf_pool[*index];
			slot.pin_count += 1;
			slot.last_access = Instant::now();

			Ok(PageRef::new(slot.page_lock.clone(), *index))
		} else {
			// need to read page from disk
			let raw_page = match dm.read_page(page_id) {
				Ok(p) => p,
				Err(e) => {
					return Err(Error::wrap(InternalError, "Error while pinning page", e));
				}
			};

			let index = if self.buf_pool.len() < self.pool_size {
				// pool isn't full yet
				let index = self.buf_pool.len();
				self.buf_pool.push(BufPoolSlot {
					page_id,
					page_lock: Arc::new(RwLock::new(raw_page)),
					pin_count: 0,
					last_access: Instant::now(),
					dirty: false,
				});

				index
			} else {
				// pool is full, choosing a slot to replace

				let index = loop {
					// choosing which slot to evict
					let mut choice: Option<usize> = None;
					let mut earliest: Instant = *INIT_TIME;
					for (i, slot) in self.buf_pool.iter().enumerate() {
						if slot.pin_count == 0 && slot.last_access > earliest {
							earliest = slot.last_access;
							choice = Some(i);
						}
					}
					if let Some(choice) = choice {
						break choice;
					}
					sleep(Duration::from_millis(POLL_SLEEP_TIME));
				};

				// removing old page
				let old_page_id = self.buf_pool[index].page_id;
				self.page_index.remove(&old_page_id);

				self.buf_pool[index] = BufPoolSlot {
					page_id,
					page_lock: Arc::new(RwLock::new(raw_page)),
					pin_count: 0,
					last_access: Instant::now(),
					dirty: false,
				};

				index
			};

			self.page_index.insert(page_id, index);
			Ok(PageRef::new(self.buf_pool[index].page_lock.clone(), index))
		}
	}

	/// Pins a page in the buffer pool, consuming a `PageRef`
	pub fn unpin(&mut self, page_ref: PageRef, dm: &mut DiskManager) -> Result<(), Error> {
		let slot = &mut self.buf_pool[page_ref.index];
		slot.pin_count = slot.pin_count.saturating_sub(1);
		Ok(())
	}
}
