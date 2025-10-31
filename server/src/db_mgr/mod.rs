use std::{
	collections::HashMap,
	sync::{LazyLock, Mutex, Weak},
};

use crate::*;
use db::Database;

/// Global database manager
pub const DB_MANAGER: LazyLock<Mutex<DatabaseManager>> = LazyLock::new(|| {
	Mutex::new(DatabaseManager {
		connected_dbs: HashMap::new(),
	})
});

/// Manages open databases
pub struct DatabaseManager {
	/// Map of open databases, by name
	connected_dbs: HashMap<String, Weak<Database>>,
}

impl DatabaseManager {}
