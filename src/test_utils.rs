use std::env;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::*;

/// Convenience alias for the result of a unit test
pub type TestResult = Result<(), Error>;

/// Creates a temp directory at `./test_output/[test_name]`, overwriting any existing
/// files and changes the currect working directory to be there
///
/// Returns the path to the directory
pub fn use_test_dir(test_name: &str) -> Result<PathBuf, std::io::Error> {
	let path = Path::new(env!("TEST_OUTPUT_DIR")).join(test_name);
	if path.exists() {
		std::fs::remove_dir_all(path.clone()).unwrap();
	}

	std::fs::create_dir_all(path.clone())?;
	std::env::set_current_dir(path.clone())?;

	Ok(path)
}
