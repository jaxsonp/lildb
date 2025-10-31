use std::io::{self, Read};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ResponseContent {
	/// Positive acknowledgment
	Ok,
	/// Error while serving request
	Error(String),
	// /// Response to a query
	//QueryResponse(QueryResult),
}

impl Encodable for ResponseContent {
	fn encode(&self) -> Vec<u8> {
		let mut out: Vec<u8> = Vec::new();
		// content type
		let discriminant: u8 = match self {
			ResponseContent::Ok => 0,
			ResponseContent::Error(_) => 1,
		};
		out.extend(discriminant.encode());
		match self {
			ResponseContent::Ok => {}
			ResponseContent::Error(err_str) => {
				out.extend(err_str.encode());
			}
		}
		return out;
	}
}

impl<R: Read> Decodable<R> for ResponseContent {
	fn decode(bytes: &mut R) -> io::Result<ResponseContent> {
		let discriminant = u8::decode(bytes)?;
		Ok(match discriminant {
			0 => ResponseContent::Ok,
			1 => {
				let err_str = String::decode(bytes)?;
				ResponseContent::Error(err_str)
			}
			_ => {
				return Err(io::Error::other("Malformed response discriminant"));
			}
		})
	}
}
