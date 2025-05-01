use crate::*;
use db::*;

// offsets
const NEXT_OFFSET: usize = 0;
const PREV_OFFSET: usize = 8;
const DATA_START_OFFSET: usize = 16;

/// The raw bytes of a page, loaded into memory
///
/// Metadata format:
/// ```text
/// 0      8      16
/// | next | prev | page_data...
/// ```
///
/// * Uses little endian for applicable methods
pub struct Page {
	pub id: PageId,
	pub(super) bytes: [u8; db::PAGE_SIZE],
}
impl Page {
	pub const DATA_LEN: usize = db::PAGE_SIZE - DATA_START_OFFSET;

	pub fn next(&self) -> Result<PageId, Error> {
		self.read_u64(NEXT_OFFSET)
	}

	pub fn set_next(&mut self, next: PageId) -> Result<(), Error> {
		self.write_u64(NEXT_OFFSET, next)
	}

	pub fn prev(&self) -> Result<PageId, Error> {
		self.read_u64(PREV_OFFSET)
	}

	pub fn set_prev(&mut self, prev: PageId) -> Result<(), Error> {
		self.write_u64(PREV_OFFSET, prev)
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
	pub fn read_bytes(&self, offset: usize, length: usize) -> Result<&[u8], Error> {
		let end = offset.saturating_add(length);
		if end > PAGE_SIZE {
			return Err(Error::new(Internal, "Tried to read bytes out of bounds"));
		}
		Ok(&self.bytes[offset..end])
	}

	pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> Result<(), Error> {
		let end = offset.saturating_add(bytes.len());
		if end > PAGE_SIZE {
			return Err(Error::new(Internal, "Tried to write bytes out of bounds"));
		}
		self.bytes[offset..end].copy_from_slice(bytes);
		Ok(())
	}

	pub fn read_u32(&self, offset: usize) -> Result<u32, Error> {
		Ok(u32::from_le_bytes(
			self.read_bytes(offset, 4)?.try_into().unwrap(),
		))
	}

	pub fn write_u32(&mut self, offset: usize, val: u32) -> Result<(), Error> {
		self.write_bytes(offset, &val.to_le_bytes())?;
		Ok(())
	}

	pub fn read_u64(&self, offset: usize) -> Result<u64, Error> {
		Ok(u64::from_le_bytes(
			self.read_bytes(offset, 8)?.try_into().unwrap(),
		))
	}

	pub fn write_u64(&mut self, offset: usize, val: u64) -> Result<(), Error> {
		self.write_bytes(offset, &val.to_le_bytes())?;
		Ok(())
	}
}
