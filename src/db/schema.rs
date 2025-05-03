use std::fmt;

use serde::{Deserialize, Serialize};

use crate::*;
use db::*;

/// Represents a table's schema, columns are order-sensitive
///
/// Constructed with a builder-esque pattern
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
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
	pub fn rec_size(&self) -> usize {
		let mut size = 0;
		for col in self.cols.iter() {
			size += col.ty.size();
		}
		size
	}
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Column {
	name: String,
	ty: ColType,
	optional: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
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
	/// 8 byte float
	Float,
	/// 16 byte float
	Double,
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
			Float => 8,
			Double => 16,
		}
	}

	/// Whether column type is fixed length
	pub fn fixed_len(&self) -> bool {
		// Have not implemented non-fixed length columns yet
		true
	}
}

impl fmt::Display for ColType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use ColType::*;
		match self {
			Bool => write!(f, "bool"),
			XShort { signed } => {
				write!(f, "{} x-short", if *signed { "signed" } else { "unsigned" })
			}
			Short { signed } => {
				write!(f, "{} short", if *signed { "signed" } else { "unsigned" })
			}
			Int { signed } => {
				write!(f, "{} int", if *signed { "signed" } else { "unsigned" })
			}
			Long { signed } => {
				write!(f, "{} long", if *signed { "signed" } else { "unsigned" })
			}
			XLong { signed } => {
				write!(f, "{} x-long", if *signed { "signed" } else { "unsigned" })
			}
			Float => write!(f, "float"),
			Double => write!(f, "double"),
		}
	}
}
