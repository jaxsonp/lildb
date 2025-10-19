mod config;
mod db;
mod error;
mod logging;
pub(crate) mod utils;

use std::{fs, path::Path};

use config::Config;

pub use error::DaemonError;
pub use log;

pub fn run(config_path: Option<String>) -> Result<(), DaemonError> {
	logging::initialize();
	log::info!("Logging initialized");

	let config = match config_path {
		Some(s) => {
			let path = Path::new(&s);
			log::info!("Config file path: {}", path.display());
			Config::from_file(path)?
		}
		None => {
			log::info!("No config file specified, using defaults");
			Config::default()
		}
	};

	// validating data directory path
	if !config.data_path.exists() {
		log::warn!("Creating data directory at {}", config.data_path.display());
		fs::create_dir(&config.data_path)?;
	} else if !config.data_path.is_dir() {
		return Err(DaemonError::Config(format!(
			"\"{}\" is not a directory",
			config.data_path.display()
		)));
	}

	return Ok(());
}
