use super::*;
use crate::test_utils::*;

#[test]
fn create() -> TestResult {
	let name = "create";
	let wd = use_test_dir(name)?;

	DiskManager::new(name, &wd)?;
	assert!(wd.join(name).with_extension("lildb").exists());

	Ok(())
}

#[test]
fn file_exists() -> TestResult {
	let name = "file_exists";
	let wd = use_test_dir(name)?;
	DiskManager::new(name, &wd)?;

	assert!(matches!(
		DiskManager::new(name, &wd),
		Err(Error {
			ty: ActionError,
			..
		})
	));

	Ok(())
}

#[test]
fn page_creation() -> TestResult {
	let name = "page_creation";
	let wd = use_test_dir(name)?;
	let mut dm = DiskManager::new(name, &wd)?;

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
	let name = "page_io";
	let wd = use_test_dir(name)?;
	let mut dm = DiskManager::new(name, &wd)?;
	let id = dm.new_page()?;

	let mut rand_bytes = [0u8; Page::DATA_LEN];
	{
		let mut page = dm.read_page(id)?;
		for (i, rand_byte) in rand_bytes.iter_mut().enumerate() {
			*rand_byte = rand::random::<u8>();
			page.data_mut()[i] = *rand_byte;
		}
		dm.write_page(&page)?;
	}

	let page = dm.read_page(id)?;
	assert!((0..Page::DATA_LEN).all(|i| page.data()[i] == rand_bytes[i]));

	Ok(())
}

#[test]
fn page_freeing() -> TestResult {
	let name = "page_freeing";
	let wd = use_test_dir(name)?;

	let mut dm = DiskManager::new(name, &wd)?;
	let id = dm.new_page()?;
	dm.free_page(id)?;

	Ok(())
}
