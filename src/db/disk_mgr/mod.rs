mod page;
#[cfg(test)]
mod tests;

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::*;
use db::*;
pub use page::Page;

// metadata page offsets
const DB_NAME_LEN: usize = 0;
const DB_NAME: usize = 4;
const CATALOG_PAGE_ID: usize = 256;

pub type PageId = u32;

/// Reads, writes, and creates pages in a database file
pub struct DiskManager {
	pub db_id: DatabaseId,
	pub db_name: String,
	file: File,
	n_pages: u32,
	/// Linked list of all pages that are not in use (AKA freed)
	free_list: Option<PageId>,
}

impl DiskManager {
	/// Creates a `DiskManager` on top of a NEW database, erroring if a database with the give name exists
	pub fn new<S: Into<String>>(db_name: S) -> Result<Self, Error> {
		let db_name: String = db_name.into();
		log::debug!("Creating disk manager for new db {db_name}",);
		if db_name.len() > 252 {
			return Err(Error::new(Action, "Database name is too long"));
		}
		let db_id = Database::get_id(db_name.as_str());

		let db_path = get_root_path()?.join(Database::get_id(&db_name).to_string());
		// assert that this db dir doesn't exist
		if db_path.exists() {
			return Err(Error::new(Action, "Database \"{db_name}\" exists"));
		}
		fs::create_dir(&db_path)?;

		let data_path = db_path.join("database.dat");
		let file = OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			.open(data_path)?;
		file.set_len(0)?;

		// metadata stuff
		let metadata = DbMetadata {
			name: db_name.clone(),
		};
		let metadata_path = db_path.join("metadata.toml");
		let mut metadata_file = OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(metadata_path)?;
		metadata_file.write_all(toml::to_string_pretty(&metadata).unwrap().as_bytes())?;

		Ok(DiskManager {
			db_id,
			db_name,
			file,
			n_pages: 0,
			free_list: None,
		})
	}

	/// Creates a new `DiskManager` from an existing database
	pub fn open(db_id: DatabaseId) -> Result<DiskManager, Error> {
		log::debug!("Creating disk manager on existing db (id {db_id})",);

		let db_path = get_root_path()?.join(db_id.to_string());
		// assert that this db dir exists
		if !db_path.exists() {
			return Err(Error::new(
				Internal,
				"Database with ID \"{db_id}\" does not exist",
			));
		}

		let data_path = db_path.join("database.dat");
		if !db_path.exists() {
			return Err(Error::new(Internal, "Database file missing"));
		}
		let file = OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			.open(data_path)?;

		let metadata_path = db_path.join("metadata.toml");
		if !metadata_path.exists() {
			return Err(Error::new(Internal, "Metadata file missing"));
		}
		let db_name: String =
			match toml::from_str::<DbMetadata>(fs::read_to_string(metadata_path)?.as_str()) {
				Ok(metadata) => metadata.name,
				Err(e) => {
					return Err(Error::new(
						Internal,
						format!("Error while parsing metadata file: {}", e.message()),
					));
				}
			};

		Ok(DiskManager {
			db_id,
			db_name,
			file,
			n_pages: 1,
			free_list: None,
		})
	}

	pub fn read_page(&mut self, id: PageId) -> Result<Page, Error> {
		if id >= self.n_pages {
			return Err(Error::new(Internal, "Attempted to read out-of-bounds page"));
		}

		let mut buf = [0u8; PAGE_SIZE];

		// getting the data
		self.file
			.seek(SeekFrom::Start(PAGE_SIZE as u64 * id as u64))?;
		self.file.read_exact(&mut buf)?;

		Ok(Page { id, bytes: buf })
	}

	pub fn write_page(&mut self, page: &Page) -> Result<(), Error> {
		if page.id >= self.n_pages {
			return Err(Error::new(
				Internal,
				"Attempted to write to out-of-bounds page",
			));
		}

		self.file
			.seek(SeekFrom::Start(PAGE_SIZE as u64 * page.id as u64))?;
		self.file.write_all(&page.bytes)?;
		self.file.flush()?;

		log::trace!("Writing to id {}", page.id);
		Ok(())
	}

	pub fn new_page(&mut self) -> Result<PageId, Error> {
		log::debug!("Getting new page from \"{}\" file", self.db_name);
		if let Some(id) = self.free_list {
			log::trace!("Recycling page {id} from free list");

			// take page from free list
			let mut page = self.read_page(id)?;
			self.free_list = Some(page.next()?);

			page.set_next(0)?;
			page.set_prev(0)?;
			self.write_page(&page)?;

			Ok(page.id)
		} else {
			log::trace!("Creating new page (#{})", self.n_pages);

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
				bytes: [0u8; PAGE_SIZE],
			};
			page.set_next(old_head)?;
			self.write_page(&page)?;
		}
		self.free_list = Some(id);

		Ok(())
	}
}

/// Metadata about a database, saved to disk
#[derive(Serialize, Deserialize)]
struct DbMetadata {
	name: String,
}
