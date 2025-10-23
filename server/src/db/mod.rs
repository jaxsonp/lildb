#[cfg(test)]
mod tests;

use crate::*;

/// Represents an opened database able to be connected to
pub struct DatabaseManager {
	pub name: String,
}

impl DatabaseManager {
	pub fn create(name: String) -> Result<DatabaseManager, ServerError> {
		if name.len() == 0 {
			return Err(ServerError::Database("Name must not be empty".to_string()));
		}
		for c in name.chars() {
			if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
				return Err(ServerError::Database(format!(
					"Database name \"{}\" is invalid, must only contain letters, numbers, dashes, and underscores",
					name
				)));
			}
		}

		let config = config()?;
		let name = name.to_ascii_lowercase();
		let path = config.db_path().join(name.as_str());
		if path.exists() {
			return Err(ServerError::Database(format!(
				"Database \"{}\" exists",
				name
			)));
		}

		// creating directory
		fs::create_dir(path)?;

		Ok(DatabaseManager { name })
	}
}
