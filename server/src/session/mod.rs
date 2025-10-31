use std::{
	io::{self, Write},
	net::{Shutdown, SocketAddr, TcpStream},
	sync::Arc,
};

use crate::*;
use db_mgr::DatabaseManager;
use lildb_api::{Decodable, Encodable, Request, RequestContent, ResponseContent};

pub struct Session {
	client_addr: SocketAddr,
	stream: TcpStream,
}
impl Session {
	pub fn new(stream: TcpStream) -> Session {
		let client_addr = stream
			.peer_addr()
			.expect("Failed to get session's client address");
		Session {
			stream,
			client_addr,
		}
	}

	/// Handles the session, serving requests and stuff
	pub fn serve(mut self) {
		let res = self._serve();
		match res {
			Ok(_) => {
				log::info!("Closing connection to {}", self.client_addr);
			}
			Err(e) => {
				log::error!("Closing connection to {} with error: {e}", self.client_addr);
			}
		}
		// drop self (aka close + cleanup connection)
	}

	/// Where the actual serve logic lives, wrapped so this can return a result and use the question operator
	fn _serve(&mut self) -> io::Result<()> {
		// wait for initial request
		let first_req = self.recv()?;
		if let RequestContent::InitSession { api } = first_req.content {
			log::debug!("Client API version: {api}");
			if !utils::check_api_version_compat(lildb_api::VERSION, api) {
				let err_msg = format!(
					"Incompatible client verstion: {} (server version: {})",
					api,
					lildb_api::VERSION
				);
				log::error!("{err_msg}");
				self.send(ResponseContent::Error(err_msg))?;
				return Ok(());
			}
		} else {
			log::error!("Unexpected first request: {first_req:?}");
			self.send(ResponseContent::Error("Unexpected request".to_string()))?;
			return Ok(());
		}
		self.send(ResponseContent::Ok)?;

		// session established, main loop
		loop {
			let req = self.recv()?;
			match req.content {
				RequestContent::InitSession { .. } => {
					self.send(ResponseContent::Error("Unexpected request".to_string()))?;
					log::error!("Unexpected request: {req:?}");
					return Ok(());
				}
				RequestContent::Exit => {
					log::info!("Client requested exit");
					break;
				}
				RequestContent::Query(query) => {
					self.send(ResponseContent::Ok)?;
				}
			}
		}

		Ok(())
	}

	/// Receives a request from the client
	fn recv(&mut self) -> io::Result<Request> {
		let req = Request::decode(&mut self.stream)?;
		log::trace!("Received {req:?}");
		Ok(req)
	}

	/// Send a response to the client
	///
	/// Returns the number of bytes sent
	fn send(&mut self, resp: ResponseContent) -> io::Result<usize> {
		log::trace!("Sending {resp:?}");
		self.stream.write(&resp.encode())
	}
}

impl Drop for Session {
	fn drop(&mut self) {
		log::debug!("Cleaning up session");
		let _ = self.stream.shutdown(Shutdown::Write);
	}
}
