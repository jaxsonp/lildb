use crate::{Decodable, Encodable};

#[test]
fn string_codec() {
	for _ in 0..1000 {
		let original: String = std::iter::repeat_with(fastrand::alphanumeric)
			.take(fastrand::usize(10..500))
			.collect();
		let encoded = original.encode();
		let decoded = String::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn u32_codec() {
	for _ in 0..1000 {
		let original = fastrand::u32(..);
		let encoded = original.encode();
		let decoded = u32::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}
