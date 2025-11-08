/// helper function to read a slice into an constant size array
///
/// `bytes` must have length `N`
pub fn slice_to_array<const N: usize>(bytes: &[u8]) -> [u8; N] {
	debug_assert_eq!(N, bytes.len());
	let mut buf = [0u8; N];
	buf.copy_from_slice(&bytes);
	buf
}
