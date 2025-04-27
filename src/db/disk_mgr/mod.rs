mod page;
#[cfg(test)]
mod tests;

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::*;
use db::*;
use page::Page;

const FILE_EXT: &str = "lildb";

pub type PageId = u32;

/// Reads, writes, and creates pages in a database file
///
/// Page 0 is all metadata
pub struct DiskManager {
	file: File,
	n_pages: u32,
	/// Linked list of all pages that are not in use (AKA freed)
	free_list: Option<PageId>,
}

impl DiskManager {
	/// Creates a disk manager on top of a new database, erroring if a database with the give name exists
	pub fn new(db_name: &str, wd_path: &PathBuf) -> Result<Self, Error> {
		let path = wd_path.join(db_name).with_extension(FILE_EXT);
		if path.exists() {
			return Error::err(ActionError, format!("Database \"{db_name}\" exists"));
		}

		let file = match OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			.open(path)
		{
			Ok(f) => f,
			Err(e) => {
				return Error::err(IOError, e.to_string());
			}
		};
		let n_pages: u32 = 1;

		file.set_len(db::PAGE_SIZE as u64)?;

		Ok(DiskManager {
			file,
			n_pages,
			free_list: None,
		})
	}

	pub fn read_page(&mut self, id: PageId) -> Result<Page, Error> {
		if id >= self.n_pages {
			return Error::err(
				InternalError,
				format!("Tried to read page that doesn't exist (index: {})", id),
			);
		}

		let mut buf = [0u8; PAGE_SIZE as usize];

		// getting the data
		self.file
			.seek(SeekFrom::Start(PAGE_SIZE as u64 * id as u64))?;
		self.file.read_exact(&mut buf)?;

		Ok(Page { id, raw: buf })
	}

	pub fn write_page(&mut self, page: &Page) -> Result<(), Error> {
		if page.id >= self.n_pages {
			return Error::err(
				InternalError,
				format!("Tried to write to invalid page (index: {})", page.id),
			);
		}

		self.file
			.seek(SeekFrom::Start(PAGE_SIZE as u64 * page.id as u64))?;
		self.file.write_all(&page.raw)?;

		Ok(())
	}

	pub fn new_page(&mut self) -> Result<PageId, Error> {
		if let Some(id) = self.free_list {
			// take page from free list
			let mut page = self.read_page(id)?;
			self.free_list = Some(page.next());

			page.set_next(0);
			page.set_prev(0);
			self.write_page(&page)?;

			Ok(page.id)
		} else {
			// no free pages to take, so create new one
			let id: PageId = self.n_pages;
			self.n_pages += 1;

			self.file
				.set_len(self.n_pages as u64 * db::PAGE_SIZE as u64)?;

			Ok(id)
		}
	}

	pub fn free_page(&mut self, id: PageId) -> Result<(), Error> {
		// insert page into free list
		if let Some(old_head) = self.free_list {
			let mut page = Page {
				id,
				raw: [0u8; PAGE_SIZE as usize],
			};
			page.set_next(old_head);
			self.write_page(&page)?;
		}
		self.free_list = Some(id);

		Ok(())
	}
}
