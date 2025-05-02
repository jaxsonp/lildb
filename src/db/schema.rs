use crate::*;
use db::*;

/// Represents a table's schema, columns are order-sensitive
///
/// Constructed with a builder-esque pattern
pub struct Schema {
	pub fixed_len: bool,
	pub n_cols: usize,
	cols: Vec<Column>,
}
impl Schema {
	pub fn new() -> Schema {
		Schema {
			fixed_len: true,
			n_cols: 0,
			cols: Vec::new(),
		}
	}

	/// Appends a column to a schema
	pub fn add_col(mut self, name: String, ty: ColType, optional: bool) -> Schema {
		if self.fixed_len && !ty.fixed_len() {
			self.fixed_len = false
		}
		self.n_cols += 1;
		self.cols.push(Column { name, ty, optional });
		self
	}

	/// Returns the size of a record with this schema
	pub fn max_rec_size(&self) -> usize {
		let mut size = 0;
		for col in self.cols.iter() {
			size += col.ty.size();
		}
		size
	}
}

struct Column {
	name: String,
	ty: ColType,
	optional: bool,
}

pub enum ColType {
	/// Boolean
	Bool,
	/// 1 byte int
	XShort { signed: bool },
	/// 2 byte int
	Short { signed: bool },
	/// 4 byte int
	Int { signed: bool },
	/// 8 byte int
	Long { signed: bool },
	/// 16 byte int
	XLong { signed: bool },
}
impl ColType {
	/// Size of column type in bytes
	pub fn size(&self) -> usize {
		use ColType::*;
		match self {
			Bool => 1,
			XShort { .. } => 1,
			Short { .. } => 2,
			Int { .. } => 4,
			Long { .. } => 8,
			XLong { .. } => 16,
		}
	}

	/// Whether column type is fixed length
	pub fn fixed_len(&self) -> bool {
		// Have not implemented non-fixed length columns yet
		true
	}
}
