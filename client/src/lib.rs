use std::{
	io::{self, Write},
	net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs},
};

use lildb_api::{Decodable, Encodable, Request, RequestContent, Response, ResponseContent};

/// Represents an active connection to a LilDB server
///
/// Gracefully closes the session on drop
pub struct LildbSession {
	_host: SocketAddr,
	stream: TcpStream,
}
impl LildbSession {
	/// Creates and establishes a new session with the server at `addr`
	pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<LildbSession> {
		let host = addr.to_socket_addrs()?.next();
		if host.is_none() {
			return Err(io::Error::other("Unable to resolve host address"));
		}
		let host = host.unwrap();
		log::debug!("Resolved host address: {}", host);
		let stream = TcpStream::connect(host)?;
		log::debug!("Connection established from local {}", stream.local_addr()?);

		let mut session = LildbSession {
			_host: host,
			stream,
		};

		// establish session
		let resp = session.send_and_recv(Request {
			content: RequestContent::InitSession {
				api: lildb_api::VERSION,
			},
		})?;
		match resp.content {
			ResponseContent::Ok => {}
			ResponseContent::Error(msg) => {
				return Err(io::Error::other(format!("Server denied connection: {msg}")));
			} /*_ => {
				  return Err(io::Error::other("Unexpected server response"));
			  }*/
		}
		log::info!("Session established to {host}");
		Ok(session)
	}

	/// Send a query to the server and get a response.
	pub fn query(&mut self, q: lildb_api::query::Query) -> io::Result<Response> {
		self.send_and_recv(Request {
			content: RequestContent::Query(q),
		})
	}

	/// Send a request to the server
	///
	/// Returns the number of bytes sent
	fn send(&mut self, req: Request) -> io::Result<usize> {
		log::trace!("Sending {req:?}");
		self.stream.write(&req.encode())
	}

	/// Receives and returns a response from the server
	fn recv(&mut self) -> io::Result<Response> {
		let resp = Response::decode(&mut self.stream)?;
		log::trace!("Received {resp:?}");
		Ok(resp)
	}

	/// Sends a request to the server, then receives and returns the server's response
	fn send_and_recv(&mut self, req: Request) -> io::Result<Response> {
		self.send(req)?;
		self.recv()
	}
}

impl Drop for LildbSession {
	fn drop(&mut self) {
		let _ = self.send(Request {
			content: RequestContent::Exit,
		});

		let _ = self.stream.shutdown(Shutdown::Write);
		log::info!("Session closed");
	}
}
