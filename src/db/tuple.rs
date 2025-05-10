use std::fmt;

use crate::*;
use db::*;

/// Represents a tuple with attributes in memory
///
/// Binary representation:
///
#[derive(Debug, Clone)]
pub struct Tuple {
	pub attrs: Vec<TupleAttr>,
}

impl Tuple {
	/// Constructs a new tuple from attributes
	pub fn new(attrs: Vec<TupleAttr>) -> Tuple {
		Tuple { attrs }
	}

	/// Constructs an empty tuple
	pub fn empty() -> Tuple {
		Tuple { attrs: Vec::new() }
	}

	/// Reconstructs a tuple from bytes and a schema
	pub fn from_bytes(bytes: Vec<u8>, schema: &Schema) -> Result<Tuple, Error> {
		let mut attr_offsets = Vec::new();
		for i in 0..schema.cols.len() {
			attr_offsets.push(
				u16::from_le_bytes(bytes[(i * 2)..((i + 1) * 2)].try_into().unwrap()) as usize,
			);
		}
		let mut attrs = Vec::new();
		for (i, col) in schema.cols.iter().enumerate() {
			let offset = attr_offsets[i];

			if (i < attr_offsets.len() - 1 && offset == attr_offsets[i + 1])
				|| (i == attr_offsets.len() - 1 && offset == 2 * attr_offsets.len())
			{
				// attr is empty
				if !col.opt {
					return Err(Error::new(
						Internal,
						"Tuple has missing attribute in non-optional column",
					));
				}
				attrs.push(TupleAttr::Empty);
				continue;
			}

			use ColType::*;
			attrs.push(match col.ty {
				Bool => TupleAttr::Bool(bytes[offset] != 0),
				XShort => TupleAttr::XShort(i8::from_le_bytes([bytes[offset]])),
				UXShort => TupleAttr::UXShort(u8::from_le_bytes([bytes[offset]])),
				Short => TupleAttr::Short(i16::from_le_bytes(
					bytes[offset..(offset + 2)].try_into().unwrap(),
				)),
				UShort => TupleAttr::UShort(u16::from_le_bytes(
					bytes[offset..(offset + 2)].try_into().unwrap(),
				)),
				Int => TupleAttr::Int(i32::from_le_bytes(
					bytes[offset..(offset + 4)].try_into().unwrap(),
				)),
				UInt => TupleAttr::UInt(u32::from_le_bytes(
					bytes[offset..(offset + 4)].try_into().unwrap(),
				)),
				Long => TupleAttr::Long(i64::from_le_bytes(
					bytes[offset..(offset + 8)].try_into().unwrap(),
				)),
				ULong => TupleAttr::ULong(u64::from_le_bytes(
					bytes[offset..(offset + 8)].try_into().unwrap(),
				)),
				Float => TupleAttr::Float(f32::from_le_bytes(
					bytes[offset..(offset + 4)].try_into().unwrap(),
				)),
				Double => TupleAttr::Double(f64::from_le_bytes(
					bytes[offset..(offset + 8)].try_into().unwrap(),
				)),
			});
		}
		Ok(Tuple::new(attrs))
	}

	/// Gets the tuple data as an array of bytes
	pub fn bytes(&self) -> Vec<u8> {
		// offset from tuple start where the data begins
		let data_offset = self.attrs.len() * 2;

		let mut attr_offsets: Vec<u8> = Vec::with_capacity(self.attrs.len() * 2);
		let mut attr_bytes: Vec<u8> = Vec::with_capacity(self.attrs.len() * 4);
		for attr in self.attrs.iter() {
			// getting local offset of attribute
			attr_offsets.extend(((data_offset + attr_bytes.len()) as u16).to_le_bytes());

			attr.append_bytes(&mut attr_bytes);
		}

		[attr_offsets, attr_bytes].concat()
	}
}

/// Represents a attribute in a tuple
#[derive(Debug, Clone, Copy)]
pub enum TupleAttr {
	Bool(bool),
	XShort(i8),
	UXShort(u8),
	Short(i16),
	UShort(u16),
	Int(i32),
	UInt(u32),
	Long(i64),
	ULong(u64),
	Float(f32),
	Double(f64),
	Empty,
}
impl TupleAttr {
	/// Gets the associated column type of this attribute
	pub fn ty(&self) -> Option<ColType> {
		use TupleAttr::*;
		match self {
			Bool(_) => Some(ColType::Bool),
			XShort(_) => Some(ColType::XShort),
			UXShort(_) => Some(ColType::UXShort),
			Short(_) => Some(ColType::Short),
			UShort(_) => Some(ColType::UShort),
			Int(_) => Some(ColType::Int),
			UInt(_) => Some(ColType::UInt),
			Long(_) => Some(ColType::Long),
			ULong(_) => Some(ColType::ULong),
			Float(_) => Some(ColType::Float),
			Double(_) => Some(ColType::Double),
			Empty => None,
		}
	}

	/// Gets the size of this attribute
	pub fn size(&self) -> usize {
		if let Some(ty) = self.ty() {
			ty.size()
		} else {
			0
		}
	}

	/// Appends this value's bytes onto to a vector
	pub fn append_bytes(&self, vec: &mut Vec<u8>) {
		use TupleAttr::*;
		match self {
			Bool(b) => vec.extend(u8::from(*b).to_le_bytes()),
			XShort(n) => vec.extend(n.to_le_bytes()),
			UXShort(n) => vec.extend(n.to_le_bytes()),
			Short(n) => vec.extend(n.to_le_bytes()),
			UShort(n) => vec.extend(n.to_le_bytes()),
			Int(n) => vec.extend(n.to_le_bytes()),
			UInt(n) => vec.extend(n.to_le_bytes()),
			Long(n) => vec.extend(n.to_le_bytes()),
			ULong(n) => vec.extend(n.to_le_bytes()),
			Float(n) => vec.extend(n.to_le_bytes()),
			Double(n) => vec.extend(n.to_le_bytes()),
			Empty => {}
		}
	}
}
impl fmt::Display for TupleAttr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use TupleAttr::*;
		match self {
			Bool(b) => write!(f, "{b}"),
			XShort(n) => write!(f, "{n}"),
			UXShort(n) => write!(f, "{n}"),
			Short(n) => write!(f, "{n}"),
			UShort(n) => write!(f, "{n}"),
			Int(n) => write!(f, "{n}"),
			UInt(n) => write!(f, "{n}"),
			Long(n) => write!(f, "{n}"),
			ULong(n) => write!(f, "{n}"),
			Float(n) => write!(f, "{n}"),
			Double(n) => write!(f, "{n}"),
			Empty => write!(f, ""),
		}
	}
}
