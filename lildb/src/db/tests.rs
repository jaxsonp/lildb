use super::*;
use utils::testing::*;

#[test]
fn create() -> TestResult {
	setup_test();
	let config = config()?;
	let db_name = create_db_name!();
	let db = DatabaseManager::create(db_name)?;

	assert!(config.db_path().join(db.name).is_dir(),);
	Ok(())
}

#[test]
fn create_invalid_name() -> TestResult {
	setup_test();
	assert!(DatabaseManager::create("".to_string()).is_err());
	assert!(DatabaseManager::create("hello.world".to_string()).is_err());
	assert!(DatabaseManager::create("hello\0world".to_string()).is_err());
	Ok(())
}

#[test]
fn create_already_exists() -> TestResult {
	setup_test();
	let db_name = create_db_name!();
	let _db = DatabaseManager::create(db_name.clone())?;

	assert!(DatabaseManager::create(db_name).is_err());
	Ok(())
}
