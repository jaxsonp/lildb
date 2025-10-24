use std::{
	io::Write,
	net::{Shutdown, TcpStream},
};

use crate::*;
use db::DatabaseManager;
use lildb_api::{Decodable, Encodable, Request, RequestContent, Response};

pub struct Session {
	stream: TcpStream,
	_cur_db: Option<DatabaseManager>,
}
impl Session {
	pub fn new(stream: TcpStream) -> Session {
		Session {
			stream,
			_cur_db: None,
		}
	}

	pub fn handle(mut self) -> io::Result<()> {
		//let mut cur_db: Option<DatabaseManager> = None;

		// wait for initial request
		let first_req = self.recv()?;
		if let RequestContent::InitSession { api } = first_req.content {
			log::debug!("Client API version: {api}");
			if !util::check_api_version_compat(lildb_api::VERSION, api) {
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
			self.send(Response::Error("Unexpected request".to_string()))?;
			return Ok(());
		}

		self.send(Response::Ok)?;

		std::thread::sleep(std::time::Duration::from_secs_f32(3.0));

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
		log::debug!("Cleaning up session ");
		self.stream
			.shutdown(Shutdown::Both)
			.expect("Error while closing tcp stream");
	}
}
