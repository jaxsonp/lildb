use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;

use log::LevelFilter;
use walkdir::WalkDir;

use crate::*;

/// Convenience alias for the result of a unit test
pub type TestResult = Result<(), Error>;

static TESTING_INIT: Once = Once::new();

/// Shorthand to initialize testing stuff
pub fn init_testing() {
	TESTING_INIT.call_once(|| {
		// init logging
		env_logger::builder()
			.is_test(true)
			.filter_level(LevelFilter::max())
			.init();
		log::info!("Logging initialized");

		let out_path = Path::new(env!("TEST_OUTPUT_DIR"));
		fs::create_dir_all(out_path).expect("Error while creating test dir");

		// gotta do this because all the disk manager unit tests were toctou-ing each other when
		// creating this dir (bruh moment)
		let data_path = out_path.join("data");
		if !data_path.exists() {
			fs::create_dir(data_path).expect("Error while creating test data dir");
		}

		// getting existing .lildb files and deleting them
		let mut to_delete: Vec<PathBuf> = Vec::new();
		for dir_entry in WalkDir::new(out_path) {
			let path = dir_entry
				.expect("Error while traversing out dir")
				.into_path();
			if let Some(ext) = path.extension() {
				if ext == "lildb" {
					to_delete.push(path);
				}
			}
		}
		log::info!("Deleting {} existing db files", to_delete.len());
		for path in to_delete {
			fs::remove_file(path).expect("Failed to delete test file");
		}
	});
}
