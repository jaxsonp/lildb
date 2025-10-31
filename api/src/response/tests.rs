use super::*;

#[test]
fn ok_response_codec() {
	let original = Response {
		content: ResponseContent::Ok,
	};
	let encoded = original.encode();
	let decoded = Response::decode(&mut encoded.as_slice()).unwrap();
	assert_eq!(original, decoded);
}
#[test]
fn err_response_codec() {
	for _ in 0..100 {
		let err_msg: String = std::iter::repeat_with(fastrand::alphanumeric)
			.take(fastrand::usize(10..200))
			.collect();
		let original = Response {
			content: ResponseContent::Error(err_msg),
		};
		let encoded = original.encode();
		let decoded = Response::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}
