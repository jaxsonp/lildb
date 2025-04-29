use super::*;
use crate::test_utils::*;

#[test]
fn access() -> TestResult {
	let buf_mgr = BufferManager::access()?;
	drop(buf_mgr);

	let buf_mgr2 = BufferManager::access()?;
	drop(buf_mgr2);

	Ok(())
}

// helper
fn get_test_dm(name: &str) -> Result<DiskManager, Error> {
	let wd = use_test_dir(name)?;
	DiskManager::new(name, &wd)
}

#[test]
fn buffering() -> TestResult {
	let name = "buffering";
	let mut dm = get_test_dm(name)?;
	let mut bm = BufferManager::new(10);

	let mut ids: Vec<PageId> = Vec::new();
	for _ in 0..20 {
		let id = dm.new_page()?;
		let page_ref = bm.pin(id, &mut dm)?;
		{
			let mut page = page_ref.get_mut()?;
			let data = page.data_mut();
			data[0] = id as u8 * 10;
		}
		bm.unpin(page_ref, &mut dm)?;

		ids.push(id);
	}

	for id in ids {
		let page_ref = bm.pin(id, &mut dm)?;
		{
			let page = page_ref.get()?;
			let data = page.data();
			assert_eq!(data[0], id as u8 * 10);
		}
		bm.unpin(page_ref, &mut dm)?;
	}
	Ok(())
}
