mod page;

use std::fs::File;
#[cfg(unix)]
use std::os::unix::fs::FileExt;
#[cfg(windows)]
use std::os::windows::fs::FileExt;

use crate::*;
use page::Page;
pub use page::PageId;

/// Manages file operations
pub struct DiskManager {
	file: LockedFile,
	n_pages: u32,
}
impl DiskManager {
	/// Instantiates a disk manager with a database file
	pub fn new(f: File) -> Result<DiskManager> {
		let n_pages = f.metadata()?.len() as u32;
		let file = LockedFile::new(f);
		Ok(Self { file, n_pages })
	}

	/// Initializes a file to be a database and creates an owning Disk Manager
	pub fn init_db(f: File) -> Result<DiskManager> {
		let n_pages = 1;
		f.set_len((PAGE_SIZE as u64) * (n_pages as u64))?;

		let mut dm = DiskManager {
			file: LockedFile::new(f),
			n_pages,
		};

		let empty_page = Page::new_empty(0);
		dm.flush_page(&empty_page)?;

		Ok(dm)
	}

	/// Reads a page from file
	fn read_page(&mut self, id: PageId) -> Result<Page> {
		if id > self.n_pages {
			return Err(Error::Internal(
				"Tried to read page out of bounds".to_string(),
			));
		}

		// read the bytes
		let mut page_buf = [0u8; PAGE_SIZE];
		self.file.read(&mut page_buf, id * (PAGE_SIZE as u32))?;

		Ok(Page::from_bytes(page_buf, id)?)
	}

	/// Writes a page to file
	fn flush_page(&mut self, page: &Page) -> Result<()> {
		let mut bytes = page.to_bytes()?;
		self.file.write(&mut bytes, page.id * (PAGE_SIZE as u32))
	}
}

/// A wrapper struct around a file, ensuring that file is always accessed behind a synchronized lock
struct LockedFile {
	f: File,
}
impl LockedFile {
	pub fn new(f: File) -> LockedFile {
		LockedFile { f }
	}

	/// Writes the whole buffer at the given offset
	pub fn write(&mut self, buf: &mut [u8], offset: u32) -> Result<()> {
		self.f.lock()?;
		self.f.write_all_at(buf, offset as u64)?;
		self.f.unlock()?;
		Ok(())
	}

	/// Reads enough bytes to fill buffer, from offset
	pub fn read(&mut self, buf: &mut [u8], offset: u32) -> Result<()> {
		self.f.lock_shared()?;
		self.f.read_exact_at(buf, offset as u64)?;
		self.f.unlock()?;
		Ok(())
	}
}
