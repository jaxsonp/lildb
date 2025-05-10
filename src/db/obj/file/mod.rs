mod heapfile;

use crate::*;
use db::*;

/// Describes a collection of tuples
pub trait DbFile {
	/// Insert tuple into this file, returning the resulting tuple ID
	fn insert(&mut self, rec: Tuple) -> Result<TupleId, Error>;
	/// Delete tuple from this file
	fn delete(&mut self, rid: TupleId) -> Result<(), Error>;
	/// Get a scan that iterates through all tuples in this file
	fn get_scan(&self) -> Result<Box<dyn QepNode>, Error>;
}
