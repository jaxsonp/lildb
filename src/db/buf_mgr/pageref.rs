use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::*;
use db::*;

/// A reference to a pinned page in the buffer manager
///
/// A `PageRef` instance will unpin itself when dropped
pub struct PageRef {
	lock: Arc<RwLock<Page>>,
	/// Index of this page in the buffer pool
	pub(super) index: usize,
}

impl PageRef {
	pub(super) fn new(lock: Arc<RwLock<Page>>, index: usize) -> PageRef {
		PageRef { lock, index }
	}

	pub fn get(&self) -> Result<RwLockReadGuard<Page>, Error> {
		Ok(self.lock.read()?)
	}

	pub fn get_mut(&self) -> Result<RwLockWriteGuard<Page>, Error> {
		// mark page as dirty
		let mut buf_mgr = BufferManager::access()?;
		buf_mgr.buf_pool[self.index].dirty = true;
		drop(buf_mgr);

		Ok(self.lock.write()?)
	}
}
