use super::DATA_SIZE;
use crate::{db::record::*, *};

const FREE_MARKER: u8 = 0;
const OCCUPIED_MARKER: u8 = 1;

/// Wrapper around page, with methods to insert/manage fixed length records
///
/// Data layout:
/// ```txt
/// |free_slots|slot_markers...|record1|record2|...
/// 0          2               ^ records_offset
/// ```
pub struct FixedLenPageView<'a> {
	data: &'a mut [u8; DATA_SIZE],
	schema: &'a Schema,
	n_slots: u16,
	rec_size: u16,
	records_offset: u16,
}
impl<'a> FixedLenPageView<'a> {
	/// Opens a `FixedLenPage` view on a page's data
	pub fn new(data: &'a mut [u8; DATA_SIZE], schema: &'a Schema) -> Result<FixedLenPageView<'a>> {
		let Some(rec_size) = schema.size() else {
			return Err(Error::Internal(
				"Attempted to instantiate fixed len page with non-fixed len schema".to_string(),
			));
		};
		let n_slots = (DATA_SIZE as u16 - 2) / (rec_size + 1);
		let records_offset = 2 + n_slots;
		Ok(FixedLenPageView {
			data,
			schema,
			n_slots,
			rec_size,
			records_offset,
		})
	}

	/// Initialize a page as a fixed length page
	pub fn init(&mut self) {
		self.set_free_slots(self.n_slots);
	}

	/// Attempts to insert a record into this page, returning `Ok(None)` if there is no space, or the slot number if insertion was successful
	///
	/// **WARNING**: This function assumes the record conforms to the configured schema
	pub fn insert_record(&mut self, rec: Record) -> Result<Option<u16>> {
		debug_assert!(self.schema.validate(&rec));

		let free_slots = self.get_free_slots();
		if free_slots == 0 {
			return Ok(None);
		}

		// finding empty slot
		let mut slot = 0;
		while !self.is_slot_free(slot) {
			slot += 1;
		}

		// set the slot as occupied
		self.set_slot_occupied(slot);
		self.set_free_slots(free_slots - 1);

		// write record data
		let rec_bytes = rec.to_bytes();
		debug_assert_eq!(rec_bytes.len() as u16, self.rec_size);
		let offset = (self.records_offset + (slot * self.rec_size)) as usize;
		if offset + rec_bytes.len() > DATA_SIZE {
			return Err(Error::Internal(
				"Out of bounds page fixed-len record write".to_string(),
			));
		}
		self.data[offset..(offset + rec_bytes.len())].copy_from_slice(&rec_bytes.as_slice());

		Ok(Some(slot))
	}

	/// Gets a record from the page with the assumption that it exists
	pub fn retrieve_record(&mut self, slot: u16) -> Result<Record> {
		debug_assert!(!self.is_slot_free(slot));

		// getting bytes
		let offset = (self.records_offset + (slot * self.rec_size)) as usize;
		let mut rec_bytes = vec![0u8; self.rec_size as usize];
		rec_bytes.copy_from_slice(&self.data[offset..(offset + (self.rec_size as usize))]);

		// building record
		let rec = Record::from_bytes(rec_bytes.as_slice(), self.schema);

		// marking as free
		self.set_slot_free(slot);
		self.set_free_slots(self.get_free_slots() + 1);

		return Ok(rec);
	}

	#[inline]
	fn get_free_slots(&self) -> u16 {
		u16::from_le_bytes(self.data[0..2].try_into().unwrap())
	}

	#[inline]
	fn set_free_slots(&mut self, value: u16) {
		self.data[0..2].copy_from_slice(&value.to_le_bytes());
	}

	#[inline]
	fn is_slot_free(&self, slot: u16) -> bool {
		self.data[2 + slot as usize] == FREE_MARKER
	}

	#[inline]
	fn set_slot_free(&mut self, slot: u16) {
		self.data[2 + slot as usize] = FREE_MARKER;
	}

	#[inline]
	fn set_slot_occupied(&mut self, slot: u16) {
		self.data[2 + slot as usize] = OCCUPIED_MARKER;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::db::disk::page::Page;

	#[test]
	pub fn insert_and_retrieve() {
		let schema = Schema::new()
			.with(ValueType::U32)
			.with(ValueType::U32)
			.with(ValueType::I32);
		let mut page = Page::new_empty(0);
		let mut view = FixedLenPageView::new(&mut page.data, &schema)
			.expect("Failed to create fixed len page view");
		view.init();
		let n_slots = view.get_free_slots();

		// inserting till page is full
		let mut inserted: Vec<u16> = Vec::new();
		let mut i: u32 = 0;
		loop {
			let rec = Record::new()
				.item(Value::U32(i))
				.item(Value::U32(i * 2))
				.item(Value::I32(-(i as i32)));
			println!("Trying to insert record {rec:?}");
			if let Some(slot) = view.insert_record(rec).expect("Insertion failed") {
				inserted.push(slot);
				println!("\tInserted into slot {slot}");
			} else {
				println!("\tFailed");
				break;
			}
			i += 1;
		}

		assert_eq!(inserted.len(), n_slots as usize);

		// "reopen" and read
		drop(view);
		let mut view = FixedLenPageView::new(&mut page.data, &schema)
			.expect("Failed to create fixed len page view");
		i = 0;
		for slot in inserted {
			println!("Retreiving record in slot {slot}");
			let rec = view
				.retrieve_record(slot)
				.expect("Failed to retreive record");
			println!("\tGot {rec:?}");
			assert_eq!(
				rec,
				Record::new()
					.item(Value::U32(i))
					.item(Value::U32(i * 2))
					.item(Value::I32(-(i as i32)))
			);
			i += 1;
		}
	}
}
