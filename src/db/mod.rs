mod buf_mgr;
mod disk_mgr;

use crate::*;
use buf_mgr::BufferManager;
use disk_mgr::{DiskManager, PageId};

/// Page size, bytes
const PAGE_SIZE: u32 = 4096;
/// Max length of a database name
const MAX_DB_NAME_LEN: usize = 249;

/// Instance of a loaded database
pub struct LoadedDB {
	pub(self) disk_mgr: DiskManager,
	pub(self) buf_mgr: BufferManager,
}
impl LoadedDB {
	//pub fn create(name: String) -> DB {}

	fn validate_db_name(name: &str) -> Result<(), Error> {
		if !name
			.chars()
			.all(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_')
		{
			Error::err(ValidationError, format!("Invalid database name '{name}': Name must only contain letters, numbers, dashes, and underscores"))
		} else if name.len() > MAX_DB_NAME_LEN {
			Error::err(
				ValidationError,
				format!(
                    "Invalid database name '{name}': Length must be at most {MAX_DB_NAME_LEN} characters"
                ),
			)
		} else {
			Ok(())
		}
	}
}
