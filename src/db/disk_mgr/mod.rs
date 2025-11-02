use std::fs::File;

use crate::*;

/// Manages file operations
pub struct DiskManager {
	f: File,
	n_pages: u32,
}
impl DiskManager {
	/// Instantiates a disk manager with a database file
	pub fn new(f: File) -> Result<DiskManager> {
		let n_pages = f.metadata()?.len() as u32;
		Ok(Self { f, n_pages })
	}

	/// Initializes a file to be a database and creates an owning Disk Manager
	pub fn init_db(f: File) -> Result<DiskManager> {
		let n_pages = 1;
		f.set_len((PAGE_SIZE * n_pages) as u64)?;
		Ok(Self { f, n_pages })
	}
}
