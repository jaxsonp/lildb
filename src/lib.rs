#![allow(dead_code)]
mod db;
mod error;
pub mod query;
mod util;

use db::LilDbConnection;
use error::{Error, Result};

/// Internal page size, in bytes
const PAGE_SIZE: usize = 8_192;

/// Open a new connection to the database at the path specified with default options
pub fn open<P: Into<std::path::PathBuf>>(db: P) -> Result<LilDbConnection> {
	LilDbOpts::default().open(db)
}

/// Optional options to specify when opening a connection to a DB
#[derive(Clone, Copy)]
pub struct LilDbOpts {
	/// Create the database if it does not exist
	create: bool,
}

impl LilDbOpts {
	pub fn open<P: Into<std::path::PathBuf>>(&self, db: P) -> Result<LilDbConnection> {
		Ok(LilDbConnection::open_db(db.into(), *self)?)
	}
}

impl Default for LilDbOpts {
	fn default() -> Self {
		Self { create: true }
	}
}
