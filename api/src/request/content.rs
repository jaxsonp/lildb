use crate::{Decode, Encode};

use std::io;

#[derive(Debug)]
/// Content of messages sent to the server
///
/// Will eventually populate with authentication and session management messages
pub enum RequestContent {
	/// First message send, to agree upon API version
	Connect {
		api: (u32, u32, u32),
	},
	/// Sent before closing the connection, for graceful exiting
	Exit,
	Query(String),
}

impl Encode for RequestContent {
	fn encode(&self) -> Vec<u8> {
		let mut out: Vec<u8> = Vec::new();
		// content type
		out.push(match self {
			RequestContent::Connect { .. } => 0,
			RequestContent::Exit => 1,
			RequestContent::Query(_) => 2,
		});
		// content specific data
		match &self {
			RequestContent::Connect { api } => {
				// just send the semver numbers
				out.extend_from_slice(&api.0.to_le_bytes());
				out.extend_from_slice(&api.1.to_le_bytes());
				out.extend_from_slice(&api.2.to_le_bytes());
			}
			RequestContent::Exit => {
				// no data
			}
			RequestContent::Query(query_str) => {
				// send query string length then utf8 bytes
				let bytes = query_str.as_bytes();
				out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
				out.extend_from_slice(bytes);
			}
		};
		return out;
	}
}

impl<R: io::Read> Decode<R> for RequestContent {
	fn decode(mut stream: R) -> io::Result<RequestContent> {
		let mut discriminant = [0u8; 1];
		stream.read_exact(&mut discriminant)?;

		match discriminant[0] {
			0 => {
				// connect
				let mut buf = [0u8; 4];
				stream.read_exact(&mut buf).unwrap();
				let maj = u32::from_le_bytes(buf);
				stream.read_exact(&mut buf).unwrap();
				let min = u32::from_le_bytes(buf);
				stream.read_exact(&mut buf).unwrap();
				let patch = u32::from_le_bytes(buf);
				Ok(RequestContent::Connect {
					api: (maj, min, patch),
				})
			}
			1 => {
				// exit
				Ok(RequestContent::Exit)
			}
			2 => {
				// Query: length (u32 LE) then UTF-8 bytes
				let mut len_buf = [0u8; 4];
				stream.read_exact(&mut len_buf).unwrap();
				let len = u32::from_le_bytes(len_buf) as usize;
				let mut v = vec![0u8; len];
				if len > 0 {
					stream.read_exact(&mut v[..]).unwrap();
				}
				let s = String::from_utf8(v).map_err(|e| io::Error::other(e.to_string()))?;
				Ok(RequestContent::Query(s))
			}
			_ => {
				// invalid discriminant
				Err(io::Error::other("Malformed request content"))
			}
		}
	}
}
