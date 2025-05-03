mod page;
#[cfg(test)]
mod tests;

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::*;
use db::*;
pub use page::Page;

pub type PageId = u32;

/// Reads, writes, and creates pages in a database file
pub struct DiskManager {
	pub db_id: DatabaseId,
	pub metadata: DbFileMetadata,
	/// Number of pages in this database
	n_pages: u32,
	/// Open database file
	file: File,
	/// Path to this database's directory
	path: PathBuf,
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

		Ok(DiskManager {
			db_id,
			metadata: DbFileMetadata {
				lil_db_version: env!("CARGO_PKG_VERSION").to_owned(),
				name: db_name.clone(),
			},
			n_pages: 0,
			file,
			path: db_path,
			free_list: None,
		})
	}

	/// Creates a new `DiskManager` from an existing database
	pub fn reopen(db_id: DatabaseId) -> Result<DiskManager, Error> {
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
		let file = OpenOptions::new().read(true).write(true).open(data_path)?;

		// asserting the file is the right size
		let file_len = file.metadata().unwrap().len();
		let n_pages = (file_len / PAGE_SIZE as u64) as u32;

		let metadata_path = db_path.join("metadata.dat");
		if !metadata_path.exists() {
			return Err(Error::new(Internal, "Metadata file missing"));
		}
		let metadata_file = OpenOptions::new().read(true).open(metadata_path.clone())?;
		let metadata: DbFileMetadata = match rmp_serde::decode::from_read(metadata_file) {
			Ok(m) => m,
			Err(e) => {
				return Err(Error::new(
					Internal,
					format!("Error while decoding metadata: {e}"),
				));
			}
		};

		let db_name: String = metadata.name;
		if metadata.lil_db_version != env!("CARGO_PKG_VERSION") {
			return Err(Error::new(
				Config,
				"Failed to load database saved with incompatible version",
			));
		}

		Ok(DiskManager {
			db_id,
			metadata: DbFileMetadata {
				lil_db_version: env!("CARGO_PKG_VERSION").to_owned(),
				name: db_name.clone(),
			},
			n_pages,
			file,
			path: db_path,
			free_list: None,
		})
	}

	/// Reads a page from disk, returning the raw bytes
	pub fn read_page(&mut self, id: PageId) -> Result<[u8; 4096], Error> {
		if id >= self.n_pages {
			return Err(Error::new(Internal, "Attempted to read out-of-bounds page"));
		}

		let mut buf = [0u8; PAGE_SIZE];

		// getting the data
		self.file
			.seek(SeekFrom::Start(PAGE_SIZE as u64 * id as u64))?;
		self.file.read_exact(&mut buf)?;

		Ok(buf)
	}

	/// Writes a page's raw bytes to disk
	pub fn write_page(&mut self, id: PageId, bytes: &[u8; PAGE_SIZE]) -> Result<(), Error> {
		if id >= self.n_pages {
			return Err(Error::new(
				Internal,
				"Attempted to write to out-of-bounds page",
			));
		}

		self.file
			.seek(SeekFrom::Start(PAGE_SIZE as u64 * id as u64))?;
		self.file.write_all(bytes)?;
		self.file.flush()?;
		Ok(())
	}

	/// Creates space for a new blank page, returning its ID
	pub fn new_page(&mut self) -> Result<PageId, Error> {
		// no free pages to take, so create new one
		let id: PageId = self.n_pages;
		log::debug!(
			"creating new page from \"{}\" file, id {id}",
			self.metadata.name
		);
		self.n_pages += 1;

		self.file
			.set_len(self.n_pages as u64 * db::PAGE_SIZE as u64)?;

		Ok(id)
	}

	pub fn free_page(&mut self, id: PageId) -> Result<(), Error> {
		todo!();
	}
}
impl Drop for DiskManager {
	fn drop(&mut self) {
		log::trace!("Dropping dm \"{}\" (id {})", self.metadata.name, self.db_id,);
		// flushing metadata
		let metadata_path = self.path.join("metadata.dat");
		let mut metadata_file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true) // overwrite
			.open(metadata_path)
			.unwrap();
		rmp_serde::encode::write(&mut metadata_file, &self.metadata).unwrap();
	}
}

/// Metadata about a database file, saved to disk
#[derive(Serialize, Deserialize)]
pub struct DbFileMetadata {
	pub lil_db_version: String,
	pub name: String,
}
