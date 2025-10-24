mod codec;
mod request;
mod response;
pub(crate) mod utils;

/// API semver version (major, minor, patch)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ApiVersion(pub u32, pub u32, pub u32);

impl fmt::Display for ApiVersion {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}.{}.{}", self.0, self.1, self.2)
	}
}

/// LilDB API version
pub const VERSION: ApiVersion = ApiVersion(0, 1, 0);

use std::fmt;

pub use codec::{Decodable, Encodable};
pub use request::{Request, RequestContent};
pub use response::{QueryResult, Response};
