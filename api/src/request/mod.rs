mod content;
#[cfg(test)]
mod tests;

use std::io::{self, Read};

pub use content::RequestContent;

use crate::*;

/// A request sent to the server
#[derive(Debug, PartialEq)]
pub struct Request {
	pub content: RequestContent,
}

impl Encodable for Request {
	fn encode(&self) -> Vec<u8> {
		return self.content.encode();
	}
}
impl<R: Read> Decodable<R> for Request {
	fn decode(bytes: &mut R) -> io::Result<Request> {
		let content = RequestContent::decode(bytes)?;
		return Ok(Request { content });
	}
}
