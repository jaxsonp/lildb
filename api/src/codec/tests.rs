use crate::{Decodable, Encodable};

const N_TESTS: usize = 1000;

#[test]
fn string_codec() {
	for _ in 0..N_TESTS {
		let original: String = std::iter::repeat_with(fastrand::alphanumeric)
			.take(fastrand::usize(10..500))
			.collect();
		let encoded = original.encode();
		let decoded = String::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn u8_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::u8(..);
		let encoded = original.encode();
		let decoded = u8::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn u16_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::u16(..);
		let encoded = original.encode();
		let decoded = u16::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn u32_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::u32(..);
		let encoded = original.encode();
		let decoded = u32::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn u64_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::u64(..);
		let encoded = original.encode();
		let decoded = u64::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn i8_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::i8(..);
		let encoded = original.encode();
		let decoded = i8::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn i16_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::i16(..);
		let encoded = original.encode();
		let decoded = i16::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn i32_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::i32(..);
		let encoded = original.encode();
		let decoded = i32::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn i64_codec() {
	for _ in 0..N_TESTS {
		let original = fastrand::i64(..);
		let encoded = original.encode();
		let decoded = i64::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn f32_codec() {
	for _ in 0..N_TESTS {
		let original = (fastrand::f32() - 0.5) * 1_000_000.0;
		let encoded = original.encode();
		let decoded = f32::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn f64_codec() {
	for _ in 0..N_TESTS {
		let original = (fastrand::f64() - 0.5) * 1_000_000_000.0;
		let encoded = original.encode();
		let decoded = f64::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn option_codec() {
	todo!()
}

#[test]
fn vec_codec() {
	todo!()
}
