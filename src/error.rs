use std::fmt;

#[derive(Debug)]
pub struct Error {
	pub(crate) ty: ErrorType,
	pub(crate) msg: String,
	pub(crate) inner: Option<Box<Error>>,
}
impl Error {
	pub fn new<S: ToString>(ty: ErrorType, msg: S) -> Error {
		Error {
			ty,
			msg: msg.to_string(),
			inner: None,
		}
	}

	/// "Wrap" another error with a new error
	pub fn wrap<S: ToString, E: Into<Error>>(ty: ErrorType, msg: S, inner: E) -> Error {
		Error {
			ty,
			msg: msg.to_string(),
			inner: Some(Box::new(inner.into())),
		}
	}
}
impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error {
			ty: ErrorType::IOError,
			msg: e.to_string(),
			inner: None,
		}
	}
}
impl<T> From<std::sync::PoisonError<T>> for Error {
	fn from(e: std::sync::PoisonError<T>) -> Error {
		Error {
			ty: ErrorType::ConcurrencyError,
			msg: e.to_string(),
			inner: None,
		}
	}
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(inner) = &self.inner {
			write!(f, "{}: {}\n │ {}", self.ty, self.msg, inner)
		} else {
			write!(f, "{}: {}", self.ty, self.msg)
		}
	}
}

#[derive(Debug)]
pub enum ErrorType {
	/// Errors from invalid user inputs
	ValidationError,
	/// Errors that the user shouldn't be dealing with
	InternalError,
	/// Errors from disk IO
	IOError,
	/// Errors from concurrency (threads/sync stuff)
	ConcurrencyError,
	/// Errors from a user action
	ActionError,
}
impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ErrorType::ValidationError => write!(f, "ValidationError"),
			ErrorType::InternalError => write!(f, "InternalError"),
			ErrorType::IOError => write!(f, "IOError"),
			ErrorType::ConcurrencyError => write!(f, "ConcurrencyError"),
			ErrorType::ActionError => write!(f, "ActionError"),
		}
	}
}
