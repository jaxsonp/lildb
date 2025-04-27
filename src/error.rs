use std::fmt;

#[derive(Debug)]
pub struct Error {
	pub(crate) ty: ErrorType,
	pub(crate) msg: String,
}
impl Error {
	pub fn new<S: ToString>(ty: ErrorType, msg: S) -> Error {
		Error {
			ty,
			msg: msg.to_string(),
		}
	}
	pub fn err<S: ToString, T>(ty: ErrorType, msg: S) -> Result<T, Error> {
		Result::Err(Error::new(ty, msg))
	}
}
impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error {
			ty: ErrorType::IOError,
			msg: e.to_string(),
		}
	}
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}", self.ty, self.msg)
	}
}

#[derive(Debug)]
pub enum ErrorType {
	/// Errors from invalid user inputs
	ValidationError,
	/// Errors from disk IO
	IOError,
	/// Errors from a user action
	ActionError,
	/// Errors that the user shouldn't be dealing with
	InternalError,
}
impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ErrorType::ValidationError => write!(f, "ValidationError"),
			ErrorType::IOError => write!(f, "IOError"),
			ErrorType::ActionError => write!(f, "ActionError"),
			ErrorType::InternalError => write!(f, "InternalError"),
		}
	}
}
