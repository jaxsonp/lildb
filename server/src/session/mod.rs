use std::{
	io::{self, Write},
	net::{Shutdown, TcpStream},
};

use crate::*;
use db_mgr::DatabaseManager;
use lildb_api::{Decodable, Encodable, Request, RequestContent, Response};

pub struct Session {
	stream: TcpStream,
}
impl Session {
	pub fn new(stream: TcpStream) -> Session {
		Session { stream }
	}

	pub fn handle(mut self) -> io::Result<()> {
		//let mut cur_db: Option<DatabaseManager> = None;

		// wait for initial request
		let first_req = self.recv()?;
		if let RequestContent::InitSession { api } = first_req.content {
			log::debug!("Client API version: {api}");
			if !utils::check_api_version_compat(lildb_api::VERSION, api) {
				let err_msg = format!(
					"Incompatible client verstion: {} (server version: {}",
					api,
					lildb_api::VERSION
				);
				log::error!("{err_msg}");
				self.send(Response::Error(err_msg))?;
				return Ok(());
			}
		} else {
			log::error!("Unexpected first request: {first_req:?}");
			self.send(Response::Error("Unexpected request".to_string()))?;
			return Ok(());
		}
		self.send(Response::Ok)?;

		// session established, main loop
		loop {
			let req = self.recv()?;
			match req.content {
				RequestContent::InitSession { .. } => {
					self.send(Response::Error("Unexpected request".to_string()))?;
					log::error!("Unexpected request: {req:?}");
					return Ok(());
				}
				RequestContent::Exit => {
					log::info!("Client requested exit");
					break;
				}
				RequestContent::Query(_) => todo!(),
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
	fn send(&mut self, resp: Response) -> io::Result<usize> {
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
