use crate::*;
use db::*;

const FREE_SLOT_COUNT_OFFSET: usize = 0;
const NEXT_FREE_SLOT_PTR_OFFSET: usize = 2;
const RECORDS_OFFSET: usize = 4;

/// Wrapper around pages for use in a heapfile
///
/// ## Fixed length records
///
/// For fixed length schemas, records are stored as illustrated below:
///
/// ```text
/// 0                2               4            N              N+2
/// | free_slots_cnt | next_free_ptr | record ... | next_free_ptr | record ... | ... |
/// ```
///
/// ## Variable length records
///
/// Not implemented yet
pub struct HfPage<'a> {
	page: Page,
	schema: &'a Schema,
}
impl<'a> HfPage<'a> {
	pub fn new(page: Page, schema: &'a Schema) -> Result<HfPage<'a>, Error> {
		if schema.fixed_len {
			page.write_u16(
				FREE_SLOT_COUNT_OFFSET,
				((Page::DATA_LEN - size_of::<u16>()) / (schema.rec_size() + size_of::<u16>()))
					as u16,
			)?;
		} else {
			todo!();
		}
		Ok(HfPage { page, schema })
	}

	pub fn count_free_slots(&self) -> Result<u16, Error> {
		self.page.read_u16(FREE_SLOT_COUNT_OFFSET)
	}

	// literally me when rust doesn't have inheritance
	pub fn next(&self) -> Result<PageId, Error> {
		self.page.next()
	}
	pub fn set_next(&mut self, next: PageId) -> Result<(), Error> {
		self.page.set_next(next)
	}
	pub fn prev(&self) -> Result<PageId, Error> {
		self.page.prev()
	}
	pub fn set_prev(&mut self, prev: PageId) -> Result<(), Error> {
		self.page.set_prev(prev)
	}
}
