mod content;
mod query_res;
#[cfg(test)]
mod tests;

use std::io::{self, Read};

use crate::{Decodable, Encodable};
pub use content::ResponseContent;
pub use query_res::QueryResult;

/// Response from the server
#[derive(Debug, PartialEq, Eq)]
pub struct Response {
	pub content: ResponseContent,
}

impl Encodable for Response {
	fn encode(&self) -> Vec<u8> {
		return self.content.encode();
	}
}
impl<R: Read> Decodable<R> for Response {
	fn decode(bytes: &mut R) -> io::Result<Response> {
		let content = ResponseContent::decode(bytes)?;
		return Ok(Response { content });
	}
}
