mod page;
#[cfg(test)]
mod tests;

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::{Mutex, RwLock};

use serde::{Deserialize, Serialize};

use crate::*;
use db::*;
pub use page::Page;

pub type PageId = u32;

/// Reads, writes, and creates pages in a database file
pub struct DiskManager {
	pub db_id: DatabaseId,
	pub metadata: DbFileMetadata,
	/// Open database file
	file: Mutex<File>,
	/// Number of pages
	n_pages: RwLock<u32>,
	/// Path to this database's directory
	path: PathBuf,
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

		let metadata = DbFileMetadata {
			lil_db_version: env!("CARGO_PKG_VERSION").to_owned(),
			name: db_name.clone(),
		};
		let metadata_path = db_path.join("metadata.dat");
		let mut metadata_file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true) // overwrite
			.open(metadata_path)?;
		metadata_file
			.write_all(metadata.as_bytes()?.as_slice())
			.map_err(|e| Error::wrap(Internal, "Error while writing file metadata", e))?;

		Ok(DiskManager {
			db_id,
			metadata,
			file: Mutex::new(file),
			n_pages: RwLock::new(0),
			path: db_path,
			free_list: None,
		})
	}

	/// Creates a new `DiskManager` from an existing database
	pub fn reopen(db_id: DatabaseId) -> Result<DiskManager, Error> {
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
		let file = OpenOptions::new().read(true).write(true).open(data_path)?;

		let metadata_path = db_path.join("metadata.dat");
		if !metadata_path.exists() {
			return Err(Error::new(
				Internal,
				format!(
					"Metadata file missing (at {})",
					metadata_path.to_str().unwrap()
				),
			));
		}
		let mut metadata_file = OpenOptions::new()
			.read(true)
			.open(metadata_path)
			.map_err(|e| Error::wrap(Internal, "Error while opening metadata file", e))?;

		let mut buf = Vec::new();
		metadata_file.read_to_end(&mut buf)?;
		let metadata = DbFileMetadata::from_bytes(buf.as_slice())?;

		let db_name: String = metadata.name;
		if metadata.lil_db_version != env!("CARGO_PKG_VERSION") {
			return Err(Error::new(
				Config,
				"Failed to load database saved with incompatible version",
			));
		}

		let n_pages = file.metadata()?.len() as usize / PAGE_SIZE;

		Ok(DiskManager {
			db_id,
			metadata: DbFileMetadata {
				lil_db_version: env!("CARGO_PKG_VERSION").to_owned(),
				name: db_name.clone(),
			},
			file: Mutex::new(file),
			n_pages: RwLock::new(n_pages as u32),
			path: db_path,
			free_list: None,
		})
	}

	/// Reads a page from disk, returning the raw bytes
	pub fn read_page(&self, id: PageId) -> Result<[u8; 4096], Error> {
		let mut buf = [0u8; PAGE_SIZE];

		// getting the data
		let mut file = self.file.lock()?;
		file.seek(SeekFrom::Start(PAGE_SIZE as u64 * id as u64))?;
		file.read_exact(&mut buf)
			.map_err(|e| Error::wrap(Internal, "Error while reading bytes from file", e))?;

		Ok(buf)
	}

	/// Writes a page's raw bytes to disk
	pub fn write_page(&self, id: PageId, bytes: &[u8; PAGE_SIZE]) -> Result<(), Error> {
		let mut file = self.file.lock()?;
		file.seek(SeekFrom::Start(PAGE_SIZE as u64 * id as u64))?;
		file.write_all(bytes)?;
		file.flush()?;
		Ok(())
	}

	/// Creates space for a new blank page, returning its ID
	///
	/// TODO get from free list
	pub fn new_page(&self) -> Result<PageId, Error> {
		let mut page_count = self.n_pages.write()?;
		let id: PageId = *page_count;
		*page_count += 1;
		self.file
			.lock()?
			.set_len(*page_count as u64 * db::PAGE_SIZE as u64)?;

		Ok(id)
	}

	pub fn free_page(&self, id: PageId) -> Result<(), Error> {
		todo!();
	}
}
impl Drop for DiskManager {
	fn drop(&mut self) {
		log::trace!("Dropping dm \"{}\" (id {})", self.metadata.name, self.db_id,);
		// flushing metadata
	}
}

/// Metadata about a database file, saved to disk
#[derive(Serialize, Deserialize)]
pub struct DbFileMetadata {
	pub lil_db_version: String,
	pub name: String,
}
impl DbFileMetadata {
	pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
		rmp_serde::to_vec(self)
			.map_err(|e| Error::new(Internal, format!("Error while encoding metadata ({e})")))
	}

	pub fn from_bytes(bytes: &[u8]) -> Result<DbFileMetadata, Error> {
		rmp_serde::from_slice(bytes).map_err(|e| {
			Error::new(
				Internal,
				format!("Error while decoding file metadata ({e})"),
			)
		})
	}
}
