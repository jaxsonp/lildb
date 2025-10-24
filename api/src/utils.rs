use std::io::{self, Read};

/// helper function to reads bytes into an constant size array or errors
pub fn read_to_array<R: Read, const N: usize>(bytes: &mut R) -> io::Result<[u8; N]> {
	let mut buf = [0u8; N];
	bytes.read_exact(&mut buf)?;
	Ok(buf)
}
