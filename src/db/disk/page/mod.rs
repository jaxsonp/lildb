mod fixed_len;

use crate::*;

pub type PageId = u32;

pub const HEADER_SIZE: usize = 8;
pub const DATA_SIZE: usize = PAGE_SIZE - HEADER_SIZE;

/// Uniquely identifies a `Record`
///
/// **WARNING**: `RecordId`'s may not be stable (remain valid indefinitely) depending on the page wrapper that produced it
#[derive(Debug, Clone, Copy)]
pub struct RecordId {
	page_id: PageId,
	slot: u16,
}

/// A page read from disk
pub struct Page {
	pub id: PageId,
	next: PageId,
	prev: PageId,
	data: [u8; DATA_SIZE],
}
impl Page {
	pub const fn new_empty(id: PageId) -> Page {
		Page {
			id,
			next: id,
			prev: id,
			data: [0u8; DATA_SIZE],
		}
	}

	/*/// Tries to insert a record, returns record ID if successful
	pub fn insert(&mut self, rec: Record) -> Result<Option<RecordId>> {
		let rec_size = rec.size() as usize;
		if (self.hdr.n_free_slots as usize) + SLOT_SIZE < rec_size {
			return Ok(None);
		}

		let rec_bytes = rec.to_bytes();
		debug_assert_eq!(rec_bytes.len(), rec_size as usize);

		// inserting record
		let rec_offset = (self.hdr.records_offset as usize) - rec_size;
		self.data[rec_offset..(rec_offset + rec_size as usize)].copy_from_slice(&rec_bytes);
		self.hdr.n_free_slots -= rec_size as u16;
		self.hdr.records_offset = rec_offset as u16;

		// updating slot
		let offset = slot_num as usize * size_of::<u16>();
		self.data[slot_offset..(slot_offset + size_of::<u16>())]
			.copy_from_slice(&rec_offset.to_le_bytes());
		self.hdr.n_free_slots -= SLOT_SIZE as u16;

		Ok(Some(RecordId {
			page_id: self.id,
			offset: slot_num,
		}))
	}*/

	pub fn to_bytes(&self) -> Result<[u8; PAGE_SIZE]> {
		let mut buf = [0u8; PAGE_SIZE];

		// header
		buf[0..4].copy_from_slice(&self.next.to_le_bytes());
		buf[4..8].copy_from_slice(&self.prev.to_le_bytes());

		buf[HEADER_SIZE..].copy_from_slice(&self.data);

		Ok(buf)
	}

	pub fn from_bytes(bytes: [u8; PAGE_SIZE], id: PageId) -> Result<Page> {
		let next = PageId::from_le_bytes(bytes[0..4].try_into().unwrap());
		let prev = PageId::from_le_bytes(bytes[4..8].try_into().unwrap());
		let data: [u8; PAGE_SIZE - HEADER_SIZE] = bytes[HEADER_SIZE..].try_into().unwrap();
		Ok(Page {
			id,
			next,
			prev,
			data,
		})
	}
}
