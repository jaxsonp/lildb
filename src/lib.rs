mod db;

mod error;
mod macros;
#[cfg(test)]
mod test_utils;

use error::{Error, ErrorType::*};

pub struct DbConn {}
