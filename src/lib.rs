mod db;
mod error;
pub mod query;

use db::DbConnection;
use error::{Error, Result};

/// Internal page size, in bytes
const PAGE_SIZE: u32 = 8_192;

/// Open a new connection to the database at the path specified, erroring if it does not exist
pub fn open<P: Into<std::path::PathBuf>>(db: P) -> Result<DbConnection> {
	Ok(DbConnection::open_db(db.into())?)
}

/// Open a new connection to the database at the path specified, creating it if it does not exist
pub fn open_or_create<P: Into<std::path::PathBuf>>(db: P) -> Result<DbConnection> {
	let db_path: std::path::PathBuf = db.into();
	if db_path.exists() {
		return Ok(DbConnection::open_db(db_path)?);
	} else {
		return Ok(DbConnection::create_db(db_path)?);
	}
}
