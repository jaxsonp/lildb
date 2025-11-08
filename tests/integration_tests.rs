mod utils;

pub use lildb::*;
pub use utils::*;

#[test]
fn create_db() {
	let db_path = unique_db!();
	let _ = open(db_path.clone()).unwrap();
	assert!(db_path.exists())
}
