#[cfg(test)]
mod tests;

use std::io::{self, Read};

use crate::utils::read_to_array;

/// Trait to encapsulate message encoding implementation
pub trait Encodable {
	/// Turns self into a vector of bytes
	fn encode(&self) -> Vec<u8>;
}

/// Trait to encapsulate message decoding implementation
pub trait Decodable<R: Read>: Sized {
	/// Attempts to read a `Self` from the read object, erroring if unable
	fn decode(bytes: &mut R) -> io::Result<Self>;
}

impl Encodable for String {
	fn encode(&self) -> Vec<u8> {
		let mut out = Vec::with_capacity(self.len() + size_of::<u32>());
		let bytes = self.as_bytes();
		out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
		out.extend_from_slice(bytes);
		return out;
	}
}
impl<R: Read> Decodable<R> for String {
	fn decode(mut bytes: &mut R) -> io::Result<Self> {
		let len = u32::from_le_bytes(read_to_array(&mut bytes)?) as usize;
		let mut buf = vec![0u8; len];
		if len > 0 {
			bytes.read_exact(buf.as_mut_slice())?;
		}
		String::from_utf8(buf).map_err(|e| io::Error::other(e.to_string()))
	}
}

impl Encodable for u32 {
	fn encode(&self) -> Vec<u8> {
		return Vec::from(self.to_le_bytes());
	}
}
impl<R: Read> Decodable<R> for u32 {
	fn decode(bytes: &mut R) -> io::Result<Self> {
		Ok(u32::from_le_bytes(read_to_array(bytes)?))
	}
}
