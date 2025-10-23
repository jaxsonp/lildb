mod content;

use std::io;

pub use content::RequestContent;

use crate::*;

/// A request sent to the server
#[derive(Debug)]
pub struct Request {
	content: RequestContent,
}
impl Request {
	pub fn connect() -> Self {
		Request {
			content: RequestContent::Connect { api: VERSION },
		}
	}
}
impl Encode for Request {
	fn encode(&self) -> Vec<u8> {
		return self.content.encode();
	}
}

impl<R: io::Read> Decode<R> for Request {
	fn decode(stream: R) -> io::Result<Request> {
		let content = RequestContent::decode(stream)?;
		return Ok(Request { content });
	}
}
