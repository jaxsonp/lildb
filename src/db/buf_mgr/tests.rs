use std::thread;
use std::time::Duration;

use super::*;
use crate::test_utils::*;

/// Max number of threads per test
const MAX_THREADS: usize = 25;

#[test]
fn buffering() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("buf_mgr_buffering")?));

	let n = BufferManager::POOL_SIZE;
	let mut ids: Vec<PageId> = Vec::new();
	for _ in 0..(n + 5) {
		let id = dm.lock()?.new_page()?;

		let page = BufferManager::pin(id, &dm)?;
		page.write_u32(0, id * 10)?;

		ids.push(id);
	}

	for id in ids {
		let page = BufferManager::pin(id, &dm)?;
		assert_eq!(page.read_u32(0)?, id * 10);
	}

	Ok(())
}

#[test]
/// Testing not being able to pin a page when all buffer manager pool slots are taken
fn full_buf_pool() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("buf_mgr_pool_full")?));

	let n = BufferManager::POOL_SIZE;
	let mut pages: Vec<Page> = Vec::new();
	for _ in 0..n {
		let id = dm.lock()?.new_page()?;
		let page = BufferManager::pin(id, &dm)?;

		page.write_u32(0, id * 10)?;
		pages.push(page);
	}

	// background thread that tries to pin one more page, shouldn't exit until main thread unpins a page
	let new_id = dm.lock().unwrap().new_page().unwrap();
	let thread = thread::spawn(move || {
		let _page_ref = BufferManager::pin(new_id, &dm).unwrap();
	});

	thread::sleep(Duration::from_secs(1));
	assert!(!thread.is_finished());

	// unpinning last page
	let _ = pages.pop();

	thread::sleep(Duration::from_secs_f32(0.5));
	assert!(thread.is_finished());

	for page in pages {
		assert_eq!(page.read_u32(0)?, page.id * 10);
	}

	Ok(())
}

#[test]
/// Testing synchronous access on different databases simulataneously
fn sync_access() -> TestResult {
	init_testing();

	let mut threads = Vec::new();
	for i in 0..MAX_THREADS {
		let name = format!("buf_mgr_sync_access_t{i}");
		let dm = Arc::new(Mutex::new(DiskManager::new(name.clone())?));
		threads.push(
			thread::Builder::new()
				.name(name)
				.spawn(move || {
					let mut ids: Vec<PageId> = Vec::new();
					for _ in 0..5 {
						let id = dm.lock().unwrap().new_page().unwrap();
						ids.push(id);

						let page = BufferManager::pin(id, &dm).unwrap();
						page.write_u32(0, id * 10).unwrap();
					}

					for id in ids {
						let page = BufferManager::pin(id, &dm).unwrap();
						assert_eq!(page.read_u32(0).unwrap(), id * 10);
					}
				})
				.unwrap(),
		);
	}
	for t in threads {
		t.join().unwrap();
	}

	Ok(())
}

#[test]
/// Testing synchronous access to the SAME database
fn sync_db_access() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("buf_mgr_sync_db_access")?));

	let mut threads = Vec::new();
	for i in 0..MAX_THREADS {
		let dm = dm.clone();
		threads.push(
			thread::Builder::new()
				.name(format!("buf_mgr_sync_db_access_t{i}"))
				.spawn(move || {
					let mut ids: Vec<PageId> = Vec::new();
					for _ in 0..5 {
						let id = dm.lock().unwrap().new_page().unwrap();

						let page = BufferManager::pin(id, &dm).unwrap();
						page.write_u32(0, id * 10).unwrap();

						ids.push(id);
					}

					for id in ids {
						let page = BufferManager::pin(id, &dm).unwrap();
						assert_eq!(page.read_u32(0).unwrap(), id * 10);
					}
				})
				.unwrap(),
		);
	}
	for t in threads {
		t.join().unwrap();
	}

	Ok(())
}

#[test]
/// Testing synchronous access to the same pages
fn sync_page_access() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("buf_mgr_sync_page_access")?));

	let mut page_ids: Vec<PageId> = Vec::new();
	for _ in 0..10 {
		let id = dm.lock()?.new_page()?;
		let page = BufferManager::pin(id, &dm).unwrap();
		page.write_u32(0, id * 10).unwrap();

		page_ids.push(id);
	}

	let mut threads = Vec::new();
	for i in 0..MAX_THREADS {
		let dm = dm.clone();
		let page_ids = page_ids.clone();
		threads.push(
			thread::Builder::new()
				.name(format!("buf_mgr_sync_page_access_t{i}"))
				.spawn(move || {
					for id in page_ids {
						let page = BufferManager::pin(id, &dm).unwrap();
						assert_eq!(page.read_u32(0).unwrap(), id * 10);
					}
				})
				.unwrap(),
		);
	}
	for t in threads {
		t.join().unwrap();
	}

	Ok(())
}
