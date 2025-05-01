#![allow(dead_code)] // <-- TODO delete

mod db;

mod error;
mod macros;
#[cfg(test)]
mod test_utils;

use db::Database;
use error::{Error, ErrorType::*};

pub fn make_db() -> Result<Database, Error> {
	Database::new("dummy")
}

pub struct DbConn {}
impl DbConn {}
