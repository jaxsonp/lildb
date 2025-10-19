use std::{
	fmt::{self},
	io,
};

#[derive(Debug)]
pub enum DaemonError {
	Config(String),
	Io(io::Error),
}

impl fmt::Display for DaemonError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use DaemonError::*;
		match self {
			Config(s) => write!(f, "Invalid configuration: {s}"),
			Io(e) => write!(f, "{e}"),
		}
	}
}

impl From<io::Error> for DaemonError {
	fn from(e: io::Error) -> Self {
		DaemonError::Io(e)
	}
}
