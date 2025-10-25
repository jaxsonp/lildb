use std::{
	collections::HashMap,
	sync::{Arc, Weak},
};

use crate::*;
use db::Database;

/// Represents an opened database able to be connected to
pub struct DatabaseManager {
	/// Map of open databases, by name
	connected_dbs: HashMap<String, Weak<Database>>,
}

impl DatabaseManager {}
