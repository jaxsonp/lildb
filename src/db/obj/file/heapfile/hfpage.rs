use std::rc::Rc;

use crate::*;
use db::*;

const FREE_SPACE_SIZE_OFFSET: usize = 0;
const FREE_SPACE_PTR_OFFSET: usize = 2;
const N_SLOTS_OFFSET: usize = Page::DATA_LEN - 2;

const SLOT_SIZE: u16 = 4;
pub(super) const DEFAULT_FREE_SPACE_SIZE: u16 = (Page::DATA_LEN - 6) as u16;

/// Wrapper around pages for use in a heapfile
///
/// Page layout:
/// ```text
/// 0                 2                4                     -10     -6     -2        4KB
/// | free space size | free space ptr | tuple | tuple |  ...  | slot | slot | n_slots |
/// ```
///
/// Each slot:
/// ```text
/// +0            +2          +4
///  | tuple size* | tuple ptr |
/// ```
///
/// * The most significant bit of the tuple size indicates if the slot is still occupied
pub struct HfPage {
	pub id: PageId,
	pub n_slots: u16,
	page: Page,
	schema: Rc<Schema>,
}
impl HfPage {
	/// Initializes a page as a heapfile page
	pub fn new(page: Page, schema: Rc<Schema>) -> Result<HfPage, Error> {
		log::debug!("Creating heap page on page {}", page.id);
		page.write_u16(FREE_SPACE_SIZE_OFFSET, DEFAULT_FREE_SPACE_SIZE)?;
		page.write_u16(FREE_SPACE_PTR_OFFSET, 4)?;
		page.write_u16(N_SLOTS_OFFSET, 0)?;

		let id = page.id;
		Ok(HfPage {
			id,
			n_slots: 0,
			page,
			schema,
		})
	}

	/// Reopens an existing heapfile page
	pub fn open(page: Page, schema: Rc<Schema>) -> Result<HfPage, Error> {
		let id = page.id;
		Ok(HfPage {
			id,
			n_slots: page.read_u16(N_SLOTS_OFFSET)?,
			page,
			schema,
		})
	}

	/// Gets the amount of free space in this page, more specifically, the biggest size tuple that
	/// can be inserted (considering the slot space required)
	///
	/// Doesn't account for deleted tuples, as they will get consolidated eventually
	pub fn free_space(&self) -> Result<u16, Error> {
		Ok(self
			.page
			.read_u16(FREE_SPACE_SIZE_OFFSET)?
			.saturating_sub(SLOT_SIZE))
	}

	/// Returns the bytes of a tuple from a specific slot
	pub fn get_tuple_bytes(&self, slot_num: u16) -> Result<Option<Vec<u8>>, Error> {
		if slot_num >= self.n_slots {
			return Err(Error::new(
				Internal,
				"Tried to get tuple from non-existant slot",
			));
		}

		let slot_offset = (N_SLOTS_OFFSET as u16) - ((slot_num + 1) * 4);
		let tup_size = self.page.read_u16(slot_offset)?;
		if tup_size & (1 << 15) == 0 {
			return Ok(None); // occupied bit is not set
		}
		let tup_size = tup_size & !(1 << 15);
		let tup_offset = self.page.read_u16(slot_offset + 2)?;

		Ok(Some(self.page.read_bytes(tup_offset, tup_size)?))
	}

	/// Inserts a tuple in this page
	///
	/// Assumes there is space for the tuple, returning an error if not
	pub fn insert(&mut self, tuple_bytes: Vec<u8>) -> Result<TupleId, Error> {
		let tuple_size = tuple_bytes.len() as u16;
		log::trace!("inserting tuple");
		println!("tuple size: {tuple_size}");
		println!("free space: {}", self.free_space()?);
		println!("tuple size*: 0x{:04x}", tuple_size | (1 << 15));

		// updating free space
		if let Some(free_space_size) = self
			.page
			.read_u16(FREE_SPACE_SIZE_OFFSET)?
			.checked_sub(tuple_size + SLOT_SIZE)
		{
			self.page
				.write_u16(FREE_SPACE_SIZE_OFFSET, free_space_size)?;
		} else {
			return Err(Error::new(
				Internal,
				"Unsufficient free space for tuple in page",
			));
		};
		let tuple_ptr = self.page.read_u16(FREE_SPACE_PTR_OFFSET)?;
		println!("tuple ptr: 0x{tuple_ptr:04x}");
		self.page
			.write_u16(FREE_SPACE_PTR_OFFSET, tuple_ptr + tuple_size)?;

		// writing tuple data
		self.page
			.write_bytes(tuple_ptr as usize, tuple_bytes.as_slice())?;

		// new slot
		let slot_num = self.n_slots;

		let slot_offset = (N_SLOTS_OFFSET as u16) - ((slot_num + 1) * 4);
		self.page.write_u16(slot_offset, tuple_size | (1 << 15))?;
		self.page.write_u16(slot_offset + 2, tuple_ptr)?;

		self.n_slots += 1;
		self.page.write_u16(N_SLOTS_OFFSET, self.n_slots)?;

		Ok(TupleId {
			page_id: self.id,
			slot_no: slot_num,
		})
	}

	// mfw when rust doesn't have inheritance
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
