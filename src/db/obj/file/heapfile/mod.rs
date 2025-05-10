mod hfpage;
mod scan;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use rustc_hash::FxBuildHasher;

use crate::*;
use db::*;
use hfpage::HfPage;
use scan::HeapFileScan;

const SCHEMA_OFFSET: usize = 0;

/// Unordered collection of tuples
///
/// Header page contains the binary encoded schema at offset 0. The header page is first in a
/// doubly-linked list of sorted pages, with fuller pages at the back.
///
/// Header page contains the binary-encoded schema
///
/// See `HfPage` for info on how tuples are stored in pages
pub struct HeapFile {
	pub schema: Rc<Schema>,
	/// ID of the header page of this heapfile
	header_page_id: PageId,
	dm: Arc<DiskManager>,
	/// Map of the amount of free space in all pages in this heapfile
	page_space_dir: HashMap<PageId, u16, FxBuildHasher>,
}
impl HeapFile {
	/// Create a new heapfile starting on a page
	pub fn new(
		header_page_id: PageId,
		schema: Rc<Schema>,
		dm: &Arc<DiskManager>,
	) -> Result<HeapFile, Error> {
		log::debug!("Creating heapfile");

		let mut page_space_dir = HashMap::with_hasher(FxBuildHasher);

		// TODO test this
		// writing schema to file
		let schema_bytes = rmp_serde::encode::to_vec(&*schema)
			.map_err(|e| Error::new(Internal, format!("Error while encoding schema: {e}")))?;
		if schema_bytes.len() > Page::DATA_LEN {
			return Err(Error::new(Action, "Schema is too large"));
		}

		let mut header_page = BufferManager::pin(header_page_id, dm)?;
		header_page.write_bytes(SCHEMA_OFFSET, schema_bytes.as_slice())?;

		// init with empty hfpage
		let first_page_id = dm.new_page()?;
		// TODO maybe store this in memory to reduce IOs?
		header_page.set_next(first_page_id)?;
		header_page.set_prev(first_page_id)?;

		let mut first_page = HfPage::new(BufferManager::pin(first_page_id, dm)?, schema.clone())?;
		page_space_dir.insert(first_page_id, hfpage::DEFAULT_FREE_SPACE_SIZE);
		first_page.set_next(header_page_id)?;
		first_page.set_prev(header_page_id)?;

		Ok(HeapFile {
			schema,
			header_page_id,
			dm: dm.clone(),
			page_space_dir,
		})
	}
}

impl DbObject for HeapFile {
	fn open(header_page_id: PageId) -> Self {
		todo!();
	}
}

impl DbFile for HeapFile {
	fn insert(&mut self, tuple: Tuple) -> Result<TupleId, Error> {
		println!("{tuple:?}");
		let tuple_bytes = tuple.bytes();
		println!("{:?}", tuple_bytes);
		let tuple_size = tuple_bytes.len() as u16;
		// retrieve a page with space
		let mut page: HfPage = {
			let header_page = BufferManager::pin(self.header_page_id, &self.dm)?;
			let mut choice: Option<PageId> = None;
			for (page_id, free_space) in self.page_space_dir.iter() {
				if *free_space >= tuple_size {
					choice = Some(*page_id);
					break;
				}
			}
			if let Some(page_id) = choice {
				log::trace!("Inserting tuple into page {page_id}");
				HfPage::open(BufferManager::pin(page_id, &self.dm)?, self.schema.clone())?
			} else {
				log::trace!("Inserting tuple into new page");
				let new_page_id = self.dm.new_page()?;
				self.page_space_dir
					.insert(new_page_id, hfpage::DEFAULT_FREE_SPACE_SIZE);

				let mut new_page = BufferManager::pin(new_page_id, &self.dm)?;
				{
					let mut old_next = BufferManager::pin(header_page.next()?, &self.dm)?;
					old_next.set_prev(new_page_id)?;
					new_page.set_next(old_next.id)?;
				}
				header_page.set_next(new_page_id)?;
				new_page.set_prev(self.header_page_id)?;

				HfPage::new(new_page, self.schema.clone())?
			}
		};

		let tid = page.insert(tuple_bytes)?;
		self.page_space_dir.insert(tid.page_id, page.free_space()?);
		Ok(tid)
	}

	fn delete(&mut self, tid: TupleId) -> Result<(), Error> {
		todo!();
	}

	fn get_scan(&self) -> Result<Box<dyn QepNode>, Error> {
		Ok(Box::new(HeapFileScan::new(self)?))
	}
}
