use super::*;
use crate::test_utils::*;

#[test]
fn create() -> TestResult {
	start_test!();

	DiskManager::new("disk_mgr_create")?;

	Ok(())
}

#[test]
fn file_exists() -> TestResult {
	start_test!();
	let name = "disk_mgr_file_exists";
	DiskManager::new(name)?;

	assert!(DiskManager::new(name).is_err());

	Ok(())
}

#[test]
fn page_creation() -> TestResult {
	start_test!();
	let dm = DiskManager::new("disk_mgr_page_creation")?;

	let n_pages_before = *dm.n_pages.read()?;
	dm.new_page()?;
	assert_eq!(*dm.n_pages.read()?, n_pages_before + 1);
	dm.new_page()?;
	dm.new_page()?;
	dm.new_page()?;
	assert_eq!(*dm.n_pages.read()?, n_pages_before + 4);

	Ok(())
}

#[test]
fn page_io() -> TestResult {
	start_test!();
	let dm = DiskManager::new("disk_mgr_page_io")?;
	let id = dm.new_page()?;

	let mut rand_nums = Vec::new();
	{
		let mut bytes = dm.read_page(id)?;
		for i in (0..Page::DATA_LEN).step_by(4) {
			let num = rand::random::<u8>();
			bytes[i] = num;
			rand_nums.push(num);
		}
		dm.write_page(id, &bytes)?;
	}

	let bytes = dm.read_page(id)?;
	assert!((0..Page::DATA_LEN)
		.step_by(4)
		.all(|i| bytes[i] == rand_nums[i >> 2]));

	Ok(())
}
