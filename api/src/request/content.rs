use crate::{ApiVersion, Decodable, Encodable, query::Query};

use std::io::{self, Read};

/// Content of messages sent to the server
///
/// Will eventually populate with authentication and session management messages
#[derive(Debug)]
pub enum RequestContent {
	/// First message send, to agree upon API version
	InitSession { api: ApiVersion },
	/// Sent before closing the connection, for graceful exiting
	Exit,
	/// Send query
	Query(Query),
}

impl Encodable for RequestContent {
	fn encode(&self) -> Vec<u8> {
		let mut out: Vec<u8> = Vec::new();
		// content type
		out.push(match self {
			RequestContent::InitSession { .. } => 0,
			RequestContent::Exit => 1,
			RequestContent::Query(_) => 2,
		});
		// content specific data
		match &self {
			RequestContent::InitSession { api } => {
				// just send the semver numbers
				out.extend(api.0.encode());
				out.extend(api.1.encode());
				out.extend(api.2.encode());
			}
			RequestContent::Exit => {}
			RequestContent::Query(query_str) => {
				out.extend_from_slice(&query_str.encode());
			}
		};
		return out;
	}
}

impl<R: Read> Decodable<R> for RequestContent {
	fn decode(mut b: &mut R) -> io::Result<RequestContent> {
		let discriminant = u8::decode(b)?;

		match discriminant {
			0 => {
				// connect
				let maj = u32::decode(&mut b)?;
				let min = u32::decode(&mut b)?;
				let patch = u32::decode(&mut b)?;
				Ok(RequestContent::InitSession {
					api: ApiVersion(maj, min, patch),
				})
			}
			1 => {
				// exit
				Ok(RequestContent::Exit)
			}
			2 => {
				// query
				let query_str = Query::decode(&mut b)?;
				Ok(RequestContent::Query(query_str))
			}
			_ => {
				// invalid discriminant
				Err(io::Error::other("Malformed request content"))
			}
		}
	}
}
