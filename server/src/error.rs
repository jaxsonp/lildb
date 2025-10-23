use std::{
	fmt::{self},
	io,
};

/// Various errors produced, some with additional messages or wrapped errors
#[derive(Debug)]
pub enum ServerError {
	/// Internal errors, should never occur during proper operation
	Internal(String),
	/// Invalid configuration
	Config(String),
	/// File IO error
	Io(io::Error),
	/// Database related error
	Database(String),
}

impl fmt::Display for ServerError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use ServerError::*;
		match self {
			Internal(msg) => write!(f, "Internal error: {msg}"),
			Config(msg) => write!(f, "Invalid configuration: {msg}"),
			Io(e) => write!(f, "IO error: {e}"),
			Database(msg) => write!(f, "Database error: {msg}"),
		}
	}
}

impl From<io::Error> for ServerError {
	fn from(e: io::Error) -> Self {
		ServerError::Io(e)
	}
}
