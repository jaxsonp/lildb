use crate::*;
use db::*;

/// Unordered collection of records
pub struct HeapFile {
	pub schema: Schema,
}

impl DbFile for HeapFile {
	fn insert_record(tup: Record, rid: file::RecordId) -> Result<(), Error> {
		unimplemented!();
	}
	fn get_record(rid: file::RecordId) -> Result<Record, Error> {
		unimplemented!();
	}
	fn delete_record(rid: file::RecordId) -> Result<(), Error> {
		unimplemented!();
	}
}

pub struct HeapFileScan {}
impl Iterator for HeapFileScan {
	type Item = Record;

	fn next(&mut self) -> Option<Record> {
		unimplemented!();
	}
}
