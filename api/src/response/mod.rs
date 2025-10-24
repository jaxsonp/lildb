mod query_res;
#[cfg(test)]
mod tests;

use std::io::{self, Read};

pub use query_res::QueryResult;

use crate::{Decodable, Encodable};

/// Response from the server
#[derive(Debug)]
pub enum Response {
	/// Positive acknowledgment
	Ok,
	/// Error while serving request
	Error(String),
	// /// Response to a query
	//QueryResponse(QueryResult),
}

impl Encodable for Response {
	fn encode(&self) -> Vec<u8> {
		let mut out: Vec<u8> = Vec::new();
		// content type
		let discriminant: u32 = match self {
			Response::Ok => 0,
			Response::Error(_) => 1,
		};
		out.extend(discriminant.encode());
		match self {
			Response::Ok => {}
			Response::Error(err_str) => {
				out.extend(err_str.encode());
			}
		}
		return out;
	}
}

impl<R: Read> Decodable<R> for Response {
	fn decode(bytes: &mut R) -> io::Result<Response> {
		let discriminant = u32::decode(bytes)?;
		Ok(match discriminant {
			0 => Response::Ok,
			1 => {
				let err_str = String::decode(bytes)?;
				Response::Error(err_str)
			}
			_ => {
				return Err(io::Error::other("Malformed response discriminant"));
			}
		})
	}
}
