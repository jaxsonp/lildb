use std::fmt;

use serde::{Deserialize, Serialize};

use crate::*;
use db::*;

/// Represents a table's schema, columns are order-sensitive
///
/// Constructed with a builder-esque pattern
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Schema {
	pub cols: Vec<Column>,
}
impl Schema {
	pub fn new() -> Schema {
		Schema { cols: Vec::new() }
	}

	/// Appends a column to a schema
	pub fn with_col<S: ToString>(
		mut self,
		name: S,
		ty: ColType,
		opt: bool,
	) -> Result<Schema, Error> {
		let name = name.to_string();
		for col in self.cols.iter() {
			if col.name == name {
				return Err(Error::new(
					Action,
					format!("Column with name \"{name}\" already exists"),
				));
			}
		}
		self.cols.push(Column { name, ty, opt });
		Ok(self)
	}

	/// Checks if a tuple conforms to this schema
	pub fn validate_tuple(&self, tup: &Tuple) -> bool {
		if self.cols.len() != tup.attrs.len() {
			return false;
		}
		for (col, attr) in self.cols.iter().zip(tup.attrs.iter()) {
			if let Some(attr_ty) = attr.ty() {
				if col.ty != attr_ty {
					return false;
				}
			} else {
				// attr is empty
				if !col.opt {
					return false;
				}
			}
		}
		true
	}
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Column {
	/// Column name
	pub name: String,
	/// Column data type
	pub ty: ColType,
	/// Optional
	pub opt: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum ColType {
	/// Boolean
	Bool,
	/// 1 byte int
	XShort,
	/// 1 byte unsigned int
	UXShort,
	/// 2 byte int
	Short,
	/// 2 byte unsigned int
	UShort,
	/// 4 byte int
	Int,
	/// 4 byte unsigned int
	UInt,
	/// 8 byte int
	Long,
	/// 8 byte unsigned int
	ULong,
	/// 4 byte float
	Float,
	/// 8 byte float
	Double,
}
impl ColType {
	/// Size of column type in bytes
	pub fn size(&self) -> usize {
		use ColType::*;
		match self {
			Bool | XShort | UXShort => 1,
			Short | UShort => 2,
			Int | UInt | Float => 4,
			Long | ULong | Double => 8,
		}
	}
}

impl fmt::Display for ColType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use ColType::*;
		match self {
			Bool => write!(f, "bool"),
			XShort => write!(f, "x-short"),
			UXShort => write!(f, "unsigned x-short"),
			Short => write!(f, "short"),
			UShort => write!(f, "unsigned short"),
			Int => write!(f, "int"),
			UInt => write!(f, "unsigned int"),
			Long => write!(f, "long"),
			ULong => write!(f, "unsigned long"),
			Float => write!(f, "float"),
			Double => write!(f, "double"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_utils::*;

	#[test]
	fn duplicate_cols() {
		start_test!();

		use ColType::*;
		assert!(Schema::new()
			.with_col("col1", Int, false)
			.unwrap()
			.with_col("col2", Int, false)
			.unwrap()
			.with_col("col2", Float, false)
			.is_err());
	}
}
