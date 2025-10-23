use std::{
	io::Write,
	net::{Shutdown, TcpStream},
};

use crate::*;
use db::DatabaseManager;
use lildb_api::{Decode, Request};

pub fn handle_session(mut stream: TcpStream) -> Result<(), ServerError> {
	writeln!(stream, "LilDB v{}", env!("CARGO_PKG_VERSION"))?;
	writeln!(stream, "Type 'help' for help, or 'exit' to exit")?;

	let mut cur_db: Option<DatabaseManager> = None;
	loop {
		let req = Request::decode(&mut stream)?;
		log::trace!("Received and decoded request: {req:?}");
		if let Some(db) = &cur_db {
			write!(stream, "{} > ", db.name)?;
		} else {
			write!(stream, " > ")?;
		}
	}

	stream.shutdown(Shutdown::Both)?;
	Ok(())
}
