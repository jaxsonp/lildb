use std::any::type_name;
use std::sync::{Arc, Mutex};

use crate::*;
use db::*;

// offsets
const NEXT_PAGE_OFFSET: usize = 0;
const PREV_PAGE_OFFSET: usize = 4;
const DATA_START_OFFSET: usize = 8;

/// Represents a shared reference to a page loaded in the buffer pool. Each `Page` is
/// interior-mutable
///
/// data format:
/// ```text
/// 0      4      8
/// | next | prev | page data ...
/// ```
///
/// * Uses little endian for number (de)serialization
#[derive(Clone)]
pub struct Page {
	/// This page's ID in its database
	pub id: PageId,
	/// The database this page is from
	pub db_id: DatabaseId,
	/// The index in the buffer pool slot that this page is currently loaded in
	pub buf_pool_index: usize,
	/// The raw bytes of the page
	pub bytes: Arc<RwLock<[u8; db::PAGE_SIZE]>>,
	/// Shared reference to the dirty flag in this page's slot
	pub dirty_lock: Arc<Mutex<bool>>,
}
impl Page {
	pub const DATA_LEN: usize = db::PAGE_SIZE - DATA_START_OFFSET;

	pub fn next(&self) -> Result<PageId, Error> {
		self.read_u32(NEXT_PAGE_OFFSET)
	}

	pub fn set_next(&self, next: PageId) -> Result<(), Error> {
		self.write_u32(NEXT_PAGE_OFFSET, next)
	}

	pub fn prev(&self) -> Result<PageId, Error> {
		self.read_u32(PREV_PAGE_OFFSET)
	}

	pub fn set_prev(&mut self, prev: PageId) -> Result<(), Error> {
		self.write_u32(PREV_PAGE_OFFSET, prev)
	}

	pub fn read_bytes(&self, offset: usize, length: usize) -> Result<Vec<u8>, Error> {
		let end = offset.saturating_add(length);
		if end > PAGE_SIZE {
			return Err(Error::new(Internal, "Tried to read bytes out of bounds"));
		}
		let mut bytes = vec![0u8; length];
		bytes.copy_from_slice(&self.bytes.read()?[offset..end]);
		Ok(bytes)
	}

	pub fn write_bytes(&self, offset: usize, bytes: &[u8]) -> Result<(), Error> {
		let end = offset.saturating_add(bytes.len());
		if end > PAGE_SIZE {
			return Err(Error::new(Internal, "Tried to write bytes out of bounds"));
		}
		self.bytes.write()?[offset..end].copy_from_slice(bytes);
		*self.dirty_lock.lock()? = true;
		Ok(())
	}
}
impl Drop for Page {
	fn drop(&mut self) {
		BufferManager::unpin(self.buf_pool_index).expect("Error while unpinning page on drop");
	}
}

/// Helper to generate boilerplate read and write functions. $ty must have from_le_bytes and to_le_bytes
macro_rules! read_write_functions {
	{$(<$ty:ty> => ($read:ident, $write:ident))*} => {
        impl Page {
            $(pub fn $read(&self, offset: usize) -> Result<$ty, Error> {
                if offset + size_of::<$ty>() >= PAGE_SIZE {
                    return Err(Error::new(Internal, format!("Tried to read {} out of bounds", type_name::<$ty>())));
                }
                Ok(<$ty>::from_le_bytes(
                    self.read_bytes(offset, size_of::<$ty>())?
                        .try_into()
                        .unwrap(),
                ))
            }
            pub fn $write(&self, offset: usize, val: $ty) -> Result<(), Error> {
                if offset + size_of::<$ty>() >= PAGE_SIZE {
                    return Err(Error::new(Internal, format!("Tried to write {} out of bounds", type_name::<$ty>())));
                }
                *self.dirty_lock.lock()? = true;
                self.write_bytes(offset, &val.to_le_bytes())?;
                Ok(())
            }
            )*
        }
	};
}

read_write_functions! {
	<u8>   => (read_u8,   write_u8)
	<u16>  => (read_u16,  write_u16)
	<u32>  => (read_u32,  write_u32)
	<u64>  => (read_u64,  write_u64)
	<u128> => (read_u128, write_u128)
	<i8>   => (read_i8,   write_i8)
	<i16>  => (read_i16,  write_i16)
	<i32>  => (read_i32,  write_i32)
	<i64>  => (read_i64,  write_i64)
	<i128> => (read_i128, write_i128)
	<f32>  => (read_f32,  write_f32)
	<f64>  => (read_f64,  write_f64)
}
