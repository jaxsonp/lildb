mod file;

use crate::*;
use db::*;

pub use file::DbFile;

/// Describes a database object, e.g. a table or index.
pub trait DbObject {
	/// Open an existing object from it's header page id
	fn open(header_page_id: PageId) -> Self;
}
