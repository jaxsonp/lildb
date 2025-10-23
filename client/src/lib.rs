use std::{
	io::{self, Write},
	net::{SocketAddr, TcpStream, ToSocketAddrs},
};

use lildb_api::{Encode, Request, RequestContent};

/// Represents an active connection to a LilDB server
pub struct LildbSession {
	host: SocketAddr,
	stream: TcpStream,
}

pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<LildbSession> {
	let host = addr.to_socket_addrs()?.next();
	if host.is_none() {
		return Err(io::Error::other("Unable to resolve host address"));
	}
	let host = host.unwrap();
	log::debug!("Resolved host address: {}", host);
	let mut stream = TcpStream::connect(host)?;
	log::debug!("Connection established from {}", stream.local_addr()?);

	stream.write(Request::connect().encode().as_slice())?;

	std::thread::sleep(std::time::Duration::from_secs_f32(5.0));

	Ok(LildbSession { host, stream })
}
