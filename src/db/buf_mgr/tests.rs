use std::thread;
use std::time::Duration;

use super::*;
use crate::test_utils::*;

/// Max number of threads per test
const MAX_THREADS: usize = 25;

#[test]
fn buffering() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("buffering")?));

	let n = BufferManager::POOL_SIZE;
	let mut ids: Vec<PageId> = Vec::new();
	for _ in 0..(n + 5) {
		let id = dm.lock()?.new_page()?;

		let page_ref = BufferManager::pin(id, &dm)?;
		let mut page = page_ref.get_mut()?;
		page.write_u64(0, id * 10)?;

		ids.push(id);
	}

	for id in ids {
		let page_ref = BufferManager::pin(id, &dm)?;
		let page = page_ref.get()?;
		assert_eq!(page.read_u64(0)?, id * 10);
	}

	Ok(())
}

#[test]
/// Testing not being able to pin a page when all buffer manager pool slots are taken
fn full_buf_pool() -> TestResult {
	init_testing();
	let dm = Arc::new(Mutex::new(DiskManager::new("full_buf_pool")?));

	let n = BufferManager::POOL_SIZE;
	let mut page_refs: Vec<PageRef> = Vec::new();
	for _ in 0..n {
		let id = dm.lock()?.new_page()?;
		let page_ref = BufferManager::pin(id, &dm)?;

		{
			let mut page = page_ref.get_mut()?;
			page.write_u64(0, id * 10)?;
		}
		page_refs.push(page_ref);
	}

	// background thread that tries to pin one more page, shouldn't exit until main thread unpins a page
	let new_id = dm.lock().unwrap().new_page().unwrap();
	let thread = thread::spawn(move || {
		let _page_ref = BufferManager::pin(new_id, &dm).unwrap();
	});

	thread::sleep(Duration::from_secs(1));
	assert!(!thread.is_finished());

	// unpinning last page
	let _ = page_refs.pop();

	thread::sleep(Duration::from_secs_f32(0.5));
	assert!(thread.is_finished());

	for page_ref in page_refs {
		let page = page_ref.get()?;
		assert_eq!(page.read_u64(0)?, page_ref.page_id * 10);
	}

	Ok(())
}

#[test]
/// Testing synchronous access on different databases simulataneously
fn sync_access() -> TestResult {
	init_testing();

	let mut threads = Vec::new();
	for i in 0..MAX_THREADS {
		let name = format!("sync_access_t{i}");
		let dm = Arc::new(Mutex::new(DiskManager::new(name.clone())?));
		threads.push(
			thread::Builder::new()
				.name(name)
				.spawn(move || {
					let mut ids: Vec<PageId> = Vec::new();
					for _ in 0..5 {
						let id = dm.lock().unwrap().new_page().unwrap();

						let page_ref = BufferManager::pin(id, &dm).unwrap();
						let mut page = page_ref.get_mut().unwrap();
						page.write_u64(0, id * 10).unwrap();

						ids.push(id);
					}

					for id in ids {
						let page_ref = BufferManager::pin(id, &dm).unwrap();
						let page = page_ref.get().unwrap();

						assert_eq!(page.read_u64(0).unwrap(), id * 10);
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
	let dm = Arc::new(Mutex::new(DiskManager::new("sync_db_access")?));

	let mut threads = Vec::new();
	for i in 0..MAX_THREADS {
		let dm = dm.clone();
		threads.push(
			thread::Builder::new()
				.name(format!("buffering_sync_db_t{i}"))
				.spawn(move || {
					let mut ids: Vec<PageId> = Vec::new();
					for _ in 0..5 {
						let id = dm.lock().unwrap().new_page().unwrap();

						let page_ref = BufferManager::pin(id, &dm).unwrap();
						let mut page = page_ref.get_mut().unwrap();
						page.write_u64(0, id * 10).unwrap();

						ids.push(id);
					}

					for id in ids {
						let page_ref = BufferManager::pin(id, &dm).unwrap();
						let page = page_ref.get().unwrap();
						assert_eq!(page.read_u64(0).unwrap(), id * 10);
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
