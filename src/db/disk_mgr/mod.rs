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
// TODO make this configurable
const DATA_PATH: &str = env!("TEST_OUTPUT_DIR"); //"/var/lib/lildb/";

pub type PageId = u64;

/// Reads, writes, and creates pages in a database file
///
/// Page 0 is all metadata
///     0-4: db name len
///     4-256: db name
pub struct DiskManager {
	pub db_id: DatabaseId,
	pub db_name: String,
	file: File,
	n_pages: u64,
	/// Linked list of all pages that are not in use (AKA freed)
	free_list: Option<PageId>,
}

impl DiskManager {
	/// Creates a `DiskManager` on top of a NEW database, erroring if a database with the give name exists
	pub fn new<S: Into<String>>(db_name: S) -> Result<Self, Error> {
		let db_name: String = db_name.into();
		if db_name.len() > 252 {
			return Err(Error::new(Action, "Database name is too long"));
		}
		let db_id = Database::get_id(db_name.as_str());

		let data_path = Path::new(DATA_PATH);
		log::debug!(
			"Creating disk manager for new db {db_name} (id {db_id}) at {}",
			data_path.to_str().unwrap()
		);

		if !data_path.exists() {
			return Err(Error::new(
				Internal,
				format!("Invalid data path \"{}\"", data_path.to_str().unwrap()),
			));
		}

		// create database folder if it doesn't exist
		let path = data_path.join("data");
		if !path.exists() {
			if let Err(e) = fs::create_dir(&path) {
				return Err(Error::wrap(
					Internal,
					"Error while creating data subdirectory",
					e,
				));
			}
		}

		// check if database exists
		let path = path.join(db_id.to_string()).with_extension(FILE_EXT);
		if path.exists() {
			return Err(Error::new(Action, format!("Database \"{db_name}\" exists")));
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
					Internal,
					"Error while opening database file",
					e,
				));
			}
		};
		file.set_len(0)?;

		let mut dm = DiskManager {
			db_id,
			db_name: db_name.clone(),
			file,
			n_pages: 0,
			free_list: None,
		};

		// first page is metadata_page
		let metadata_page_id = dm.new_page()?;
		let mut metadata = dm.read_page(metadata_page_id)?;

		metadata.write_u32(0, db_name.len() as u32)?;
		metadata.write_bytes(4, db_name.as_bytes())?;
		dm.write_page(&metadata)?;

		Ok(dm)
	}

	/// Creates a new `DiskManager` from an existing database
	pub fn open(db_id: DatabaseId) -> Result<DiskManager, Error> {
		let path = Path::new(DATA_PATH)
			.join("data")
			.join(db_id.to_string())
			.with_extension(FILE_EXT);
		log::debug!(
			"Creating disk manager on existing db (id {db_id}) at {}",
			path.to_str().unwrap()
		);
		if !path.exists() {
			return Err(Error::new(
				Internal,
				format!("Can't find database with id {db_id}"),
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
					Internal,
					"Error while opening database file",
					e,
				));
			}
		};

		let mut dm = DiskManager {
			db_id,
			db_name: String::new(),
			file,
			n_pages: 1,
			free_list: None,
		};

		let metadata = dm.read_page(0)?;

		let name_len = metadata.read_u32(0)? as usize;
		dm.db_name = String::from_utf8_lossy(metadata.read_bytes(4, name_len)?).into_owned();

		Ok(dm)
	}

	pub fn read_page(&mut self, id: PageId) -> Result<Page, Error> {
		if id >= self.n_pages {
			return Err(Error::new(Internal, "Attempted to read out-of-bounds page"));
		}

		let mut buf = [0u8; PAGE_SIZE];

		// getting the data
		self.file.seek(SeekFrom::Start(PAGE_SIZE as u64 * id))?;
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
			.seek(SeekFrom::Start(PAGE_SIZE as u64 * page.id))?;
		self.file.write_all(&page.bytes)?;

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

			self.file.set_len(self.n_pages * db::PAGE_SIZE as u64)?;

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
