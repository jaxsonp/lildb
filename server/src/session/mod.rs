use std::{
	io::Write,
	net::{Shutdown, TcpStream},
};

use crate::*;
use db::DatabaseManager;

pub fn handle_session(mut stream: TcpStream) -> Result<(), DaemonError> {
	writeln!(stream, "LilDB v{}", env!("CARGO_PKG_VERSION"))?;
	writeln!(stream, "Type 'help' for help, or 'exit' to exit")?;

	let mut cur_db: Option<DatabaseManager> = None;
	loop {
		if let Some(db) = &cur_db {
			write!(stream, "{} > ", db.name)?;
		} else {
			write!(stream, " > ")?;
		}
		//let input = stream.
	}

	stream.shutdown(Shutdown::Both)?;
	Ok(())
}
