use super::*;
use crate::test_utils::*;

#[test]
fn access() -> TestResult {
	init_testing();

	let buf_mgr = buf_mgr!()?;
	drop(buf_mgr);

	let buf_mgr2 = buf_mgr!()?;
	drop(buf_mgr2);

	Ok(())
}

#[test]
fn buffering() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("buffering")?));

	let n = buf_mgr!()?.pool_size + 10;
	let mut ids: Vec<PageId> = Vec::new();
	for _ in 0..n {
		let id = dm.lock()?.new_page()?;

		let page_ref = buf_mgr!()?.pin(id, &dm)?;
		let mut page = page_ref.get_mut()?;

		let data = page.data_mut();
		data[0] = id as u8 * 10;
		println!("wrote {} to page {}", id as u8 * 10, id);

		ids.push(id);
	}

	for id in ids {
		let page_ref = buf_mgr!()?.pin(id, &dm)?;
		let page = page_ref.get()?;

		let data = page.data();
		println!("reading from page {}: {}", id, data[0]);
		assert_eq!(data[0], id as u8 * 10);
	}

	Ok(())
}
