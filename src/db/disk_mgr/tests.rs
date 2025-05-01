use super::*;
use crate::test_utils::*;

#[test]
fn create() -> TestResult {
	init_testing();

	let name = "create_abc123";
	DiskManager::new(name)?;

	Ok(())
}

#[test]
fn file_exists() -> TestResult {
	init_testing();
	let name = "file_exists";
	DiskManager::new(name)?;

	assert!(DiskManager::new(name).is_err());

	Ok(())
}

#[test]
fn page_creation() -> TestResult {
	init_testing();
	let mut dm = DiskManager::new("page_creation")?;

	let n_pages_before = dm.n_pages;
	dm.new_page()?;
	assert_eq!(dm.n_pages, n_pages_before + 1);
	dm.new_page()?;
	dm.new_page()?;
	dm.new_page()?;
	assert_eq!(dm.n_pages, n_pages_before + 4);

	Ok(())
}

#[test]
fn page_io() -> TestResult {
	init_testing();
	let mut dm = DiskManager::new("page_io")?;
	let id = dm.new_page()?;

	let mut rand_nums = Vec::new();
	{
		let mut page = dm.read_page(id)?;
		for i in (0..Page::DATA_LEN).step_by(4) {
			let num = rand::random::<u32>();
			page.write_u32(i, num)?;
			rand_nums.push(num);
		}
		dm.write_page(&page)?;
	}

	let page = dm.read_page(id)?;
	assert!((0..Page::DATA_LEN)
		.step_by(4)
		.all(|i| page.read_u32(i).unwrap() == rand_nums[i >> 2]));

	Ok(())
}

#[test]
fn page_freeing() -> TestResult {
	init_testing();

	let mut dm = DiskManager::new("page_freeing")?;
	let id = dm.new_page()?;
	dm.free_page(id)?;

	Ok(())
}
