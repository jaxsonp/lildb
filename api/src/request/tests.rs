use super::*;

#[test]
fn init_session_request_codec() {
	for _ in 0..1000 {
		let api = ApiVersion(fastrand::u32(..), fastrand::u32(..), fastrand::u32(..));
		let original = Request {
			content: RequestContent::InitSession { api },
		};
		let encoded = original.encode();
		let decoded = Request::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}

#[test]
fn exit_request_codec() {
	let original = Request {
		content: RequestContent::Exit,
	};
	let encoded = original.encode();
	let decoded = Request::decode(&mut encoded.as_slice()).unwrap();
	assert_eq!(original, decoded);
}

#[test]
fn query_request_codec() {
	for _ in 0..1000 {
		let query_str: String = std::iter::repeat_with(fastrand::alphanumeric)
			.take(fastrand::usize(10..100))
			.collect();
		let original = Request {
			content: RequestContent::Query(query_str),
		};
		let encoded = original.encode();
		let decoded = Request::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(original, decoded);
	}
}
