use super::*;

#[test]
fn init_session_request_codec() {
	for _ in 0..100 {
		let api = ApiVersion(fastrand::u32(..), fastrand::u32(..), fastrand::u32(..));
		let original = Request {
			content: RequestContent::InitSession { api },
		};
		let encoded = original.encode();
		let decoded = Request::decode(&mut encoded.as_slice()).unwrap();
		assert_eq!(format!("{original:?}"), format!("{decoded:?}"));
	}
}

#[test]
fn exit_request_codec() {
	let original = Request {
		content: RequestContent::Exit,
	};
	let encoded = original.encode();
	let decoded = Request::decode(&mut encoded.as_slice()).unwrap();
	assert_eq!(format!("{original:?}"), format!("{decoded:?}"));
}

#[test]
fn query_request_codec() {
	todo!();
}
