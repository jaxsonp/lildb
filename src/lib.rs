#![allow(dead_code, unused_variables)] // <-- TODO delete

mod db;

mod error;
mod macros;
#[cfg(test)]
mod test_utils;

use std::env;
use std::path::{Path, PathBuf};

use error::{Error, ErrorType::*};

pub struct DbConn {}
impl DbConn {}

/// Convenience function that gets the database root path from the env var, and asserts that it exists
fn get_root_path() -> Result<PathBuf, Error> {
	let root_path_str = match env::var_os("LILDB_ROOT") {
		Some(s) => s,
		None => {
			return Err(Error::new(
				Config,
				"Required env var \"LILDB_ROOT\" not set",
			));
		}
	};
	let root_path = Path::new(&root_path_str).to_path_buf();
	// assert that root path exists
	if root_path.exists() {
		Ok(root_path)
	} else {
		Err(Error::new(
			Internal,
			format!("Invalid root path \"{}\"", root_path.to_str().unwrap()),
		))
	}
}
