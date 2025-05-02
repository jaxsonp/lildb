use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::BufferManager;
use crate::*;
use db::*;

/// A reference to a pinned page in the buffer manager
///
/// Each `PageRef` instance will unpin itself from the buffer manager when dropped
pub struct PageRef {
	pub(super) page_lock: Arc<RwLock<Page>>,
	pub(super) dirty_lock: Arc<Mutex<bool>>,
	/// Index of this page in the buffer pool
	pub(super) index: usize,
	/// Database this page is from
	pub db_id: DatabaseId,
	/// Page ID
	pub page_id: PageId,
}

impl PageRef {
	/// Lock the underlying page for reading
	pub fn get(&self) -> Result<RwLockReadGuard<Page>, Error> {
		Ok(self.page_lock.read()?)
	}

	/// Lock the underlying page for writing, marking it as dirty
	pub fn get_mut(&self) -> Result<RwLockWriteGuard<Page>, Error> {
		{
			// mark page as dirty
			let mut dirty = self.dirty_lock.lock()?;
			*dirty = true;
		}

		Ok(self.page_lock.write()?)
	}
}
impl Drop for PageRef {
	fn drop(&mut self) {
		BufferManager::unpin(self.index).expect("Error while unpinning pageref on drop");
	}
}
