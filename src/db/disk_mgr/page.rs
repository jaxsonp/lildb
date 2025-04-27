use crate::*;
use db::*;

// offsets
const NEXT_OFFSET: usize = 0;
const PREV_OFFSET: usize = 4;
const FLAGS_OFFSET: usize = 8;
const DATA_START_OFFSET: usize = 12;

// flag masks
//const FLAG_MASK_INUSE: u32 = 0x01;

/// Represents a page loaded into memory
///
/// Metadata format:
/// ```text
/// 0      4      8      12
/// | next | prev | flags | ...
/// ```
pub struct Page {
	pub id: PageId,
	pub(super) raw: [u8; db::PAGE_SIZE as usize],
}
impl Page {
	pub const DATA_LEN: usize = db::PAGE_SIZE as usize - DATA_START_OFFSET;

	pub fn next(&self) -> PageId {
		self.read_u32(NEXT_OFFSET)
	}

	pub fn set_next(&mut self, next: PageId) {
		self.write_u32(NEXT_OFFSET, next)
	}

	pub fn prev(&self) -> PageId {
		self.read_u32(PREV_OFFSET)
	}

	pub fn set_prev(&mut self, prev: PageId) {
		self.write_u32(PREV_OFFSET, prev)
	}

	pub fn data(&self) -> &[u8] {
		&self.raw[DATA_START_OFFSET..]
	}

	pub fn data_mut(&mut self) -> &mut [u8] {
		&mut self.raw[DATA_START_OFFSET..]
	}

	/*pub fn check_flag(&self) -> bool {
		self.read_u32(FLAGS_OFFSET) & FLAG_MASK_INUSE != 0
	}

	pub fn set_in_use(&mut self, in_use: bool) {
		let mut flags = self.read_u32(FLAGS_OFFSET);
		if in_use {
			flags |= FLAG_MASK_INUSE;
		} else {
			flags &= !FLAG_MASK_INUSE;
		}
		self.write_u32(FLAGS_OFFSET, flags);
	}*/

	fn read_u32(&self, offset: usize) -> u32 {
		let bytes: [u8; 4] = self.raw[offset..(offset + 5)].try_into().unwrap();
		u32::from_ne_bytes(bytes)
	}

	fn write_u32(&mut self, offset: usize, val: u32) {
		let bytes: &mut [u8; 4] = &mut self.raw[offset..(offset + 5)].try_into().unwrap();
		*bytes = val.to_ne_bytes();
	}
}
