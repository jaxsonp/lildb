use std::rc::Rc;
use std::sync::Arc;

use super::{HeapFile, HfPage};
use crate::*;
use db::*;

/// A scan of all tuples in a heapfile
pub struct HeapFileScan {
	schema: Rc<Schema>,
	dm: Arc<DiskManager>,
	/// Current page
	cur_page: HfPage,
	/// Current slot in current page
	cur_slot: u16,
	/// Page Id of the header of the heapfile
	header_page_id: PageId,
	/// Total slots in the current page
	n_slots: u16,
}

impl HeapFileScan {
	pub fn new(hf: &HeapFile) -> Result<HeapFileScan, Error> {
		log::trace!("starting scan");
		let first_page_id = {
			let header = BufferManager::pin(hf.header_page_id, &hf.dm)?;
			header.next()?
		};
		let cur_page = HfPage::open(
			BufferManager::pin(first_page_id, &hf.dm)?,
			hf.schema.clone(),
		)?;
		let n_slots = cur_page.n_slots;
		Ok(HeapFileScan {
			schema: hf.schema.clone(),
			dm: hf.dm.clone(),
			cur_page,
			cur_slot: 0,
			header_page_id: hf.header_page_id,
			n_slots,
		})
	}
}
impl QepNode for HeapFileScan {
	fn schema(&self) -> Rc<Schema> {
		self.schema.clone()
	}

	fn get_next(&mut self) -> Result<Option<Tuple>, Error> {
		log::trace!("getting next");
		loop {
			log::trace!(
				"pos: page {}, slot {}/{}",
				self.cur_page.id,
				self.cur_slot,
				self.n_slots
			);
			if self.cur_slot >= self.n_slots {
				// done with this page
				let next_page_id = self.cur_page.next()?;
				if next_page_id == self.header_page_id {
					// went through all pages, iteration done
					return Ok(None);
				}
				self.cur_page = HfPage::open(
					BufferManager::pin(next_page_id, &self.dm)?,
					self.schema.clone(),
				)?;
				self.cur_slot = 0;
				self.n_slots = self.cur_page.n_slots;
			} else {
				if let Some(bytes) = self.cur_page.get_tuple_bytes(self.cur_slot)? {
					self.cur_slot += 1;
					return Ok(Some(Tuple::from_bytes(bytes, &self.schema)?));
				}
				// slot unoccupied, try again
				self.cur_slot += 1;
			}
		}
	}
}
