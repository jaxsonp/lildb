use super::*;
use crate::test_utils::*;

#[test]
fn create() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("heapfile_create")?));

	/*let header = BufferManager::pin(dm.lock()?.new_page()?, &dm)?;
	let schema = Schema::new().add_col("", ty, optional)
	let hf = HeapFile::new(header);*/

	Ok(())
}
