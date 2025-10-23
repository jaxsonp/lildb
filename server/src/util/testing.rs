use std::{path::PathBuf, sync::Once};

use crate::*;

pub type TestResult = Result<(), ServerError>;

/// Used to make sure that testing is initialized once
static INIT_TESTS: Once = Once::new();

pub fn setup_test() {
	INIT_TESTS.call_once(|| {
		// if this is the first test, set up the directory structure
		let config = Config {
			data_path: PathBuf::new().join("../test-data"),
			..Config::default()
		};

		// delete testing dir if it exits
		if config.data_path.exists() {
			fs::remove_dir_all(&config.data_path).unwrap();
		}

		crate::validate_dirs(&config).expect("Error validating directory structure");

		config::initialize_global_config(config).expect("double initialized global config");
	});
}

/// Gets the name of the current module, and formats so it can be used as a database name along with a suffix to distinguish tests
#[macro_export]
macro_rules! create_db_name {
	() => {
		format!("{}_{}", module_path!().replace("::", "_"), line!())
	};
}
