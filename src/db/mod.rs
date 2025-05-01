mod buf_mgr;
mod disk_mgr;

use std::hash::Hasher;
use std::path::Path;
use std::sync::{Arc, RwLock};

use rustc_hash::FxHasher;

use crate::*;
use buf_mgr::buf_mgr;
use disk_mgr::{DiskManager, Page, PageId};

pub type DatabaseId = u64;

/// Page size, in bytes
const PAGE_SIZE: u32 = 4096;

/// Size of buffer pool (max number of pages simulataneously in memory)
const BUF_POOL_SIZE: usize = if cfg!(test) { 10 } else { 100 };

/// Max length of a database name
const MAX_DB_NAME_LEN: usize = 249;

/// Instance of a loaded database
pub struct Database {
	pub id: DatabaseId,
	disk_mgr: DiskManager,
}
impl Database {
	pub fn new(name: &str) -> Result<Database, Error> {
		// TODO implement datapath properly
		Ok(Self {
			id: Database::get_id(name),
			disk_mgr: DiskManager::new(name)?,
		})
	}

	/// Gets a database ID from its name
	fn get_id(name: &str) -> DatabaseId {
		let mut h = FxHasher::default();
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
