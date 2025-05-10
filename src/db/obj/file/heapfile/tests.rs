use std::sync::LazyLock;

use super::*;
use crate::test_utils::*;

#[allow(non_upper_case_globals)]
static test_schema: LazyLock<Schema> = LazyLock::new(|| {
	Schema::new()
		.with_col("id", ColType::UInt, false)
		.unwrap()
		.with_col("number", ColType::Int, false)
		.unwrap()
		.with_col("square", ColType::ULong, false)
		.unwrap()
		.with_col("root", ColType::Float, true)
		.unwrap()
		.with_col("even", ColType::Bool, false)
		.unwrap()
});

/// Helper function to generate test tuples with the test schema
fn generate_test_tuple(id: u32) -> Tuple {
	let num = rand::random::<i8>() as i32;

	use TupleAttr::*;
	let tup = Tuple::new(vec![
		UInt(id),
		Int(num),
		ULong((num as i64).pow(2) as u64),
		if num >= 0 {
			Float((num as f32).sqrt())
		} else {
			Empty
		},
		Bool(num % 2 == 0),
	]);

	assert!(test_schema.validate_tuple(&tup));
	tup
}

#[test]
fn insert_small() -> TestResult {
	start_test!();
	let dm = Arc::new(DiskManager::new("heapfile_insert_small")?);

	let mut hf = HeapFile::new(dm.new_page()?, test_schema.clone().into(), &dm)?;

	for i in 0..50u32 {
		let tup = generate_test_tuple(i);
		let _ = hf.insert(tup)?;
	}

	Ok(())
}

#[test]
fn insert_large() -> TestResult {
	start_test!();
	let dm = Arc::new(DiskManager::new("heapfile_insert_large")?);

	let mut hf = HeapFile::new(dm.new_page()?, test_schema.clone().into(), &dm)?;

	for i in 0..10000u32 {
		let tup = generate_test_tuple(i);
		let _ = hf.insert(tup)?;
	}

	Ok(())
}

#[test]
fn scan() -> TestResult {
	start_test!();
	let dm = Arc::new(DiskManager::new("heapfile_scan")?);

	let mut hf = HeapFile::new(dm.new_page()?, test_schema.clone().into(), &dm)?;

	let n: u32 = 500;
	for i in 0..n {
		let tup = generate_test_tuple(i);
		hf.insert(tup)?;
	}

	let mut scan = hf.get_scan()?;
	let mut count = 0;
	while let Some(tup) = scan.get_next()? {
		assert!(test_schema.validate_tuple(&tup));

		match tup.attrs[0] {
			TupleAttr::UInt(_) => {}
			_ => panic!(),
		};
		let num = match tup.attrs[1] {
			TupleAttr::Int(n) => n,
			_ => panic!(),
		};
		match tup.attrs[2] {
			TupleAttr::ULong(sqr) => assert_eq!(sqr, (num as i64).pow(2) as u64),
			_ => {
				panic!();
			}
		}
		match tup.attrs[3] {
			TupleAttr::Float(root) => {
				assert!(num >= 0);
				assert_eq!(root, (num as f32).sqrt());
			}
			TupleAttr::Empty => {
				assert!(num < 0);
			}
			_ => panic!(),
		}
		match tup.attrs[4] {
			TupleAttr::Bool(even) => assert_eq!(even, num % 2 == 0),
			_ => panic!(),
		}

		count += 1;
	}
	assert_eq!(count, n);

	Ok(())
}
