mod page;
#[cfg(test)]
mod tests;

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::*;
use db::*;
pub use page::Page;

const FILE_EXT: &str = "lildb";
const DB_SUBDIR: &str = "data";

pub type PageId = u64;

/// Reads, writes, and creates pages in a database file
///
/// Page 0 is all metadata
pub struct DiskManager {
	pub db_id: DatabaseId,
	file: File,
	n_pages: u64,
	/// Linked list of all pages that are not in use (AKA freed)
	free_list: Option<PageId>,
}

impl DiskManager {
	/// Creates a disk manager on top of a new database, erroring if a database with the give name exists
	pub fn new(db_name: &str, data_path: &Path) -> Result<Self, Error> {
		if !data_path.exists() {
			return Err(Error::new(
				ActionError,
				format!("Invalid data path \"{}\"", data_path.to_str().unwrap()),
			));
		}

		// create database folder if it doesn't exist
		let path = data_path.join(DB_SUBDIR);
		if !path.exists() {
			if let Err(e) = fs::create_dir(&path) {
				return Err(Error::wrap(
					InternalError,
					"Error while creating data subdirectory",
					e,
				));
			}
		}

		// check if database exists
		let path = path.join(db_name).with_extension(FILE_EXT);
		if path.exists() {
			return Err(Error::new(
				ActionError,
				format!("Database \"{db_name}\" exists"),
			));
		}

		let file = match OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			.open(path)
		{
			Ok(f) => f,
			Err(e) => {
				return Err(Error::wrap(
					InternalError,
					"Error while opening database file",
					e,
				));
			}
		};

		file.set_len(db::PAGE_SIZE as u64)?;

		Ok(DiskManager {
			db_id: Database::get_id(db_name),
			file,
			n_pages: 1,
			free_list: None,
		})
	}

	pub fn read_page(&mut self, id: PageId) -> Result<Page, Error> {
		if id >= self.n_pages {
			return Err(Error::new(
				InternalError,
				"Attempted to read out-of-bounds page",
			));
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
			return Err(Error::new(
				InternalError,
				"Attempted to write to out-of-bounds page",
			));
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
