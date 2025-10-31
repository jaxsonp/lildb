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
		out.extend_from_slice(&(bytes.len() as u32).encode());
		out.extend_from_slice(bytes);
		return out;
	}
}
impl<R: Read> Decodable<R> for String {
	fn decode(bytes: &mut R) -> io::Result<Self> {
		let len = u32::decode(bytes)? as usize;
		let mut buf = vec![0u8; len];
		if len > 0 {
			bytes.read_exact(buf.as_mut_slice())?;
		}
		String::from_utf8(buf).map_err(|e| io::Error::other(e.to_string()))
	}
}

impl<T: Encodable> Encodable for Vec<T> {
	fn encode(&self) -> Vec<u8> {
		let mut out = Vec::new();
		out.extend_from_slice(&(self.len() as u32).encode());
		for item in self.iter() {
			out.extend_from_slice(&item.encode());
		}
		out
	}
}
impl<R: Read, T: Decodable<R>> Decodable<R> for Vec<T> {
	fn decode(bytes: &mut R) -> io::Result<Self> {
		let len = u32::decode(bytes)? as usize;
		let mut vec: Vec<T> = Vec::with_capacity(len);
		for _ in 0..len {
			vec.push(T::decode(bytes)?);
		}
		Ok(vec)
	}
}

impl<T: Encodable> Encodable for Option<T> {
	fn encode(&self) -> Vec<u8> {
		match self {
			Some(item) => {
				let mut out = Vec::new();
				out.push(1);
				out.extend_from_slice(&item.encode());
				out
			}
			None => vec![0u8],
		}
	}
}
impl<R: Read, T: Decodable<R>> Decodable<R> for Option<T> {
	fn decode(bytes: &mut R) -> io::Result<Self> {
		let discriminant = u8::decode(bytes)?;
		match discriminant {
			0 => Ok(Self::None),
			1 => Ok(Self::Some(T::decode(bytes)?)),
			_ => Err(io::Error::other("Malformed request content")),
		}
	}
}

/// A macro that implements `Encodable` and `Decodable` for any type that implements [to|from]_le_bytes()
macro_rules! le_bytes_types {
	($($ty:ty )*) => {
		$(
			impl Encodable for $ty {
				fn encode(&self) -> Vec<u8> {
					return Vec::from(self.to_le_bytes());
				}
			}
			impl<R: Read> Decodable<R> for $ty {
				fn decode(bytes: &mut R) -> io::Result<Self> {
					Ok(<$ty>::from_le_bytes(read_to_array(bytes)?))
				}
			}
		)*
	};
}
le_bytes_types!(
	u8 u16 u32 u64 u128
	i8 i16 i32 i64 i128
	f32 f64
);
