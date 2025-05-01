use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::*;
use db::*;

/// A reference to a pinned page in the buffer manager
///
/// A `PageRef` instance will unpin itself when dropped
pub struct PageRef {
	page_lock: Arc<RwLock<Page>>,
	dirty_lock: Arc<Mutex<bool>>,
	/// Index of this page in the buffer pool
	pub(super) index: usize,
}

impl PageRef {
	pub(super) fn new(
		page_lock: Arc<RwLock<Page>>,
		dirty_lock: Arc<Mutex<bool>>,
		index: usize,
	) -> PageRef {
		PageRef {
			page_lock,
			dirty_lock,
			index,
		}
	}

	pub fn get(&self) -> Result<RwLockReadGuard<Page>, Error> {
		Ok(self.page_lock.read()?)
	}

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
		buf_mgr!()
			.expect("Failed to access buffer manager from PageRef destructor")
			.unpin(self.index);
	}
}
