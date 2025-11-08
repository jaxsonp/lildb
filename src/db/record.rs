use std::iter;

use crate::util::slice_to_array;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
	items: Vec<Value>,
}
impl Record {
	pub fn new() -> Record {
		Record { items: Vec::new() }
	}

	pub fn item(mut self, item: Value) -> Self {
		self.items.push(item);
		self
	}

	pub fn to_bytes(self) -> Vec<u8> {
		let mut bytes = Vec::new();
		for item in self.items.into_iter() {
			bytes.extend_from_slice(&item.to_bytes());
		}
		bytes
	}

	/// Generates a record from bytes given a matching schema
	///
	/// Assumes bytes contains the right amount of bytes
	pub fn from_bytes(bytes: &[u8], schema: &Schema) -> Record {
		let mut rec = Record::new();
		let mut cur: usize = 0;
		for ty in schema.items.iter() {
			let val = match ty {
				ValueType::U32 => {
					cur += size_of::<u32>();
					Value::U32(u32::from_le_bytes(slice_to_array(
						&bytes[(cur - size_of::<u32>())..cur],
					)))
				}
				ValueType::I32 => {
					cur += size_of::<i32>();
					Value::I32(i32::from_le_bytes(slice_to_array(
						&bytes[(cur - size_of::<i32>())..cur],
					)))
				}
			};
			rec = rec.item(val);
		}
		debug_assert_eq!(cur, bytes.len());
		rec
	}
}

#[derive(PartialEq, Eq)]
pub struct Schema {
	items: Vec<ValueType>,
	size: Option<u16>,
}
impl Schema {
	pub fn new() -> Schema {
		Schema {
			items: Vec::new(),
			size: Some(0),
		}
	}

	pub fn with(mut self, ty: ValueType) -> Self {
		self.items.push(ty);
		self.size = if let Some(new_size) = ty.size() {
			self.size.map(|size| size + new_size)
		} else {
			None
		};
		self
	}

	pub fn with_n(mut self, ty: ValueType, n: usize) -> Self {
		self.items.extend(iter::repeat_n(ty, n));
		self.size = if let Some(new_size) = ty.size() {
			self.size.map(|size| size + new_size * (n as u16))
		} else {
			None
		};
		self
	}

	/// Update size with the current items
	fn recalculate_size(&mut self) {
		let mut total_size = 0;
		for item in self.items.iter() {
			match item.size() {
				Some(size) => total_size += size,
				None => {
					self.size = None;
					return;
				}
			}
		}
		self.size = Some(total_size);
	}

	/// Returns `None` if schema does not have a fixed size
	#[inline]
	pub fn size(&self) -> Option<u16> {
		self.size
	}

	/// Checks if a record conforms to this schema
	pub fn validate(&self, rec: &Record) -> bool {
		if rec.items.len() != self.items.len() {
			return false;
		}
		return iter::zip(self.items.iter(), rec.items.iter())
			.all(|(expected, actual)| actual.ty() == *expected);
	}
}
impl From<Vec<ValueType>> for Schema {
	fn from(items: Vec<ValueType>) -> Self {
		let mut s = Schema {
			items,
			size: Some(0),
		};
		s.recalculate_size();
		s
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ValueType {
	U32,
	I32,
}
impl ValueType {
	/// Returns `None` if value type is variable size
	pub const fn size(&self) -> Option<u16> {
		match self {
			ValueType::U32 | ValueType::I32 => Some(4),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
	U32(u32),
	I32(i32),
}
impl Value {
	pub const fn size(&self) -> u16 {
		match self {
			Value::U32(_) | Value::I32(_) => 4,
		}
	}

	pub const fn ty(&self) -> ValueType {
		match self {
			Value::U32(_) => ValueType::U32,
			Value::I32(_) => ValueType::I32,
		}
	}

	pub fn to_bytes(self) -> Vec<u8> {
		use Value::*;
		match self {
			U32(n) => Vec::from(n.to_le_bytes()),
			I32(n) => Vec::from(n.to_le_bytes()),
		}
	}
}
