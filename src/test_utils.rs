use std::env;
use std::fs;
use std::sync::Once;

use log::LevelFilter;

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

		let out_path = get_root_path().unwrap().join("test_output");
		if out_path.exists() {
			fs::remove_dir_all(out_path.clone())
				.expect("Error while clearing test output directory");
		}
		fs::create_dir_all(out_path.clone()).expect("Error while creating test dir");

		env::set_var("LILDB_ROOT", out_path.as_os_str());
	});
}
