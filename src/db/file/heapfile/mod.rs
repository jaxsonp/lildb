mod hfpage;
#[cfg(test)]
mod tests;

use std::sync::Mutex;

use hfpage::HfPage;

use crate::*;
use db::*;

/// Unordered collection of records
///
/// Header page contains the binary encoded schema at offset 0. The header page is first in a
/// doubly-linked list where full pages are at the end and non-full pages are at the front.
///
/// See `HfPage` for info on how records are stored in pages
pub struct HeapFile {
	pub schema: Schema,
	header_page_id: PageId,
	dm: Arc<Mutex<DiskManager>>,
}
impl HeapFile {
	/// Create a new heapfile starting on a page
	pub fn new(
		mut header_page: Page,
		schema: Schema,
		dm: &Arc<Mutex<DiskManager>>,
	) -> Result<HeapFile, Error> {
		log::debug!("Creating heapfile");

		// TODO write tests for this
		// writing schema to file
		let schema_bytes = match rmp_serde::encode::to_vec(&schema) {
			Ok(v) => v,
			Err(e) => {
				return Err(Error::new(
					Internal,
					format!("Error while encoding schema: {e}"),
				));
			}
		};
		if schema_bytes.len() > Page::DATA_LEN {
			return Err(Error::new(Action, "Schema is too large"));
		}

		header_page.write_bytes(0, schema_bytes.as_slice())?;

		// init with empty hfpage
		let first_page_id = dm.lock()?.new_page()?;
		header_page.set_next(first_page_id)?;
		header_page.set_prev(first_page_id)?;

		let mut first_page = HfPage::new(BufferManager::pin(first_page_id, dm)?, &schema)?;
		first_page.set_next(header_page.id)?;
		first_page.set_prev(header_page.id)?;

		Ok(HeapFile {
			schema,
			header_page_id: header_page.id,
			dm: dm.clone(),
		})
	}
}

impl DbFile for HeapFile {
	fn insert_record(&mut self, rec: Record, rid: RecordId) -> Result<(), Error> {
		if self.schema.fixed_len {
			todo!();
		} else {
			todo!();
		}
	}
	fn delete_record(&mut self, rid: RecordId) -> Result<(), Error> {
		if self.schema.fixed_len {
			todo!();
		} else {
			todo!();
		}
	}
}

pub struct HeapFileScan {}
impl Iterator for HeapFileScan {
	type Item = Record;

	fn next(&mut self) -> Option<Record> {
		todo!();
	}
}
