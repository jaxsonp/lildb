mod heapfile;

use crate::*;
use db::*;

use heapfile::HeapFile;

pub struct RecordId {
	page_id: PageId,
	slot_no: usize,
}

/// Trait that describes a "file", an abstraction on top of pages that manages records in a
/// collection of pages
pub trait DbFile {
	/// Insert record into file at a specific rid
	fn insert_record(&mut self, rec: Record, rid: RecordId) -> Result<(), Error>;
	/// Delete record at a specific rid
	fn delete_record(&mut self, rid: RecordId) -> Result<(), Error>;
}
