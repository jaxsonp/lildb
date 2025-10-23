mod request;
mod response;

/// API semver version (major, minor, patch)
pub type ApiVersion = (u32, u32, u32);

/// LilDB API version
pub const VERSION: ApiVersion = (0, 1, 0);

use std::io;

pub use request::{Request, RequestContent};
pub use response::{QueryResult, Response};

/// Trait to encapsulate message encoding implementation
pub trait Encode {
	fn encode(&self) -> Vec<u8>;
}

/// Trait to encapsulate message decoding
pub trait Decode<R: io::Read>: Sized {
	fn decode(stream: R) -> io::Result<Self>;
}
