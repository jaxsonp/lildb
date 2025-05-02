mod heapfile;

use crate::*;
use db::*;

pub struct RecordId {
	page_id: PageId,
	slot_no: usize,
}

/// Trait that describes a "file", an abstraction on top of pages that manages records in a
/// collection of pages
pub trait DbFile {
	/// Insert record into file at a specific rid
	fn insert_record(tup: Record, rid: RecordId) -> Result<(), Error>;
	/// Delete record at a specific rid
	fn delete_record(rid: RecordId) -> Result<(), Error>;
	/// Return the record stored at a specific rid
	fn get_record(rid: RecordId) -> Result<Record, Error>;
}
