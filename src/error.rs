use std::fmt;

#[derive(Debug)]
pub struct Error {
	pub(crate) ty: ErrorType,
	pub(crate) msg: String,
	pub(crate) inner: Option<Box<Error>>,
}
impl Error {
	pub fn new<S: ToString>(ty: ErrorType, msg: S) -> Error {
		let e = Error {
			ty,
			msg: msg.to_string(),
			inner: None,
		};
		log::error!("Error generated:\n{}", e);
		e
	}

	/// "Wrap" another error with a new error
	pub fn wrap<S: ToString, E: Into<Error>>(ty: ErrorType, msg: S, inner: E) -> Error {
		let e = Error {
			ty,
			msg: msg.to_string(),
			inner: Some(Box::new(inner.into())),
		};
		log::error!("Wrapper error generated:\n{}", e);
		e
	}
}
impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error::new(ErrorType::IO, e.to_string())
	}
}
impl<T> From<std::sync::PoisonError<T>> for Error {
	fn from(e: std::sync::PoisonError<T>) -> Error {
		Error::new(ErrorType::Concurrency, e.to_string())
	}
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(inner) = &self.inner {
			write!(f, "{}: {}\n│ {}", self.ty, self.msg, inner)
		} else {
			write!(f, "{}: {}", self.ty, self.msg)
		}
	}
}

#[derive(Debug)]
pub enum ErrorType {
	/// Errors from invalid user inputs
	Validation,
	/// Errors that the user shouldn't be dealing with
	Internal,
	/// Errors from disk IO
	IO,
	/// Errors from concurrency (threads/sync stuff)
	Concurrency,
	/// Errors from a user action
	Action,
	/// Errors from invalid configuration
	Config,
}
impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use ErrorType::*;
		match self {
			Validation => write!(f, "ValidationError"),
			Internal => write!(f, "InternalError"),
			IO => write!(f, "IOError"),
			Concurrency => write!(f, "ConcurrencyError"),
			Action => write!(f, "ActionError"),
			Config => write!(f, "ConfigError"),
		}
	}
}
