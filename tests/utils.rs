use std::{fs, path::Path, sync::Once};

pub const TEST_DIR: &str = "./test_artifacts/";

static TEST_DIR_CREATED: Once = Once::new();
/// Makes sure that the test directory has been cleaned and created
pub fn ensure_test_dir() {
	let path = Path::new(TEST_DIR);
	fs::remove_dir_all(path).unwrap();
	TEST_DIR_CREATED.call_once(|| fs::create_dir_all(path).unwrap());
}

#[macro_export]
macro_rules! unique_db {
	() => {{
		crate::utils::ensure_test_dir();
		std::path::PathBuf::from(crate::utils::TEST_DIR)
			.join(format!("{}_{}", module_path!(), line!()))
			.with_extension("ldb")
	}};
}
