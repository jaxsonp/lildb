mod buf_mgr;
mod disk_mgr;
mod obj;
mod qep;
mod schema;
mod tuple;

use std::hash::Hasher;

use crate::*;
pub(crate) use buf_mgr::BufferManager;
use disk_mgr::{DiskManager, Page, PageId};
use obj::{DbFile, DbObject};
use qep::QepNode;
use schema::{ColType, Schema};
use tuple::{Tuple, TupleAttr};

/// Page size, in bytes
const PAGE_SIZE: usize = 4096;

/// Max length of a database name
const MAX_DB_NAME_LEN: usize = 249;

pub type DatabaseId = u64;

pub struct TupleId {
	page_id: PageId,
	slot_no: u16,
}

/// Instance of a loaded database
pub struct Database {
	pub id: DatabaseId,
	disk_mgr: DiskManager,
}
impl Database {
	pub fn new(name: &str) -> Result<Database, Error> {
		Ok(Self {
			id: Database::get_id(name),
			disk_mgr: DiskManager::new(name)?,
		})
	}

	/// Gets a database ID from its name
	fn get_id(name: &str) -> DatabaseId {
		let mut h = rustc_hash::FxHasher::default();
		h.write(name.as_bytes());
		h.finish()
	}

	fn validate_db_name(name: &str) -> Result<(), Error> {
		if !name
			.chars()
			.all(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_')
		{
			Err(Error::new(Validation, format!("Invalid database name '{name}': Name must only contain letters, numbers, dashes, and underscores")))
		} else if name.len() > MAX_DB_NAME_LEN {
			Err(Error::new(
				Validation,
				format!(
                    "Invalid database name '{name}': Length must be at most {MAX_DB_NAME_LEN} characters"
                ),
			))
		} else {
			Ok(())
		}
	}
}
