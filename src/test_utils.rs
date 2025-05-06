use std::sync::{Mutex, OnceLock};

use crate::*;

/// Convenience alias for the result of a unit test
pub type TestResult = Result<(), Error>;

pub static TESTING_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

/// Helper function to reduce boilerplate in unit tests. Does the following:
/// - Initializes logging and env vars if appropriate
/// - Waits for the testing mutex to be free
macro_rules! start_test {
	() => {
		let _test_lock = TESTING_MUTEX
			.get_or_init(|| {
				// init logging
				env_logger::builder()
					.is_test(true)
					.filter_level(log::LevelFilter::max())
					.init();
				log::info!("Logging initialized");

				// move root to test_output subdir
				let out_path = get_root_path().unwrap().join("test_output");
				if out_path.exists() {
					std::fs::remove_dir_all(out_path.clone())
						.expect("Error while clearing test output directory");
				}
				std::fs::create_dir_all(out_path.clone()).expect("Error while creating test dir");
				std::env::set_var("LILDB_ROOT", out_path.as_os_str());
				std::sync::Mutex::new(())
			})
			.lock()
			.unwrap_or_else(|e| e.into_inner()); // its okay to unwrap or else because the data in
		                                   // the mutex doesn't matter
	};
}
pub(crate) use start_test;
