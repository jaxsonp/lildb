use super::*;
use crate::test_utils::*;

#[test]
fn create() -> TestResult {
	init_testing();

	DiskManager::new("create")?;

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
	init_testing();

	let mut dm = DiskManager::new("page_freeing")?;
	let id = dm.new_page()?;
	dm.free_page(id)?;

	Ok(())
}
