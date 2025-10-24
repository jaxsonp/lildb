mod config;
mod db;
mod error;
mod logging;
mod session;
pub mod util;

use std::{fs, io, net::TcpListener, path::Path, thread};

use config::Config;
use session::Session;

pub use config::config;
pub use error::ServerError;
pub use log;

/// Runs the daemon, optionally with a config file at `config_path`
pub fn run(config_path: Option<String>) -> Result<(), ServerError> {
	logging::initialize();
	log::info!("Logging initialized");

	config::initialize_global_config(match config_path {
		Some(s) => {
			let path = Path::new(&s);
			log::info!("Config file path: {}", path.display());
			Config::from_file(path)?
		}
		None => {
			log::info!("No config file specified, using defaults");
			Config::default()
		}
	})?;
	let config = config()?;

	validate_dirs(&config)?;

	let tcp_listener = TcpListener::bind((config.listen_addr, config.listen_port))?;
	log::info!("Listening on {}", tcp_listener.local_addr()?);
	loop {
		let (stream, client_addr) = tcp_listener.accept()?;
		thread::spawn(move || {
			log::info!("Accepted connection from {client_addr}");
			let res = Session::new(stream).handle();
			match res {
				Ok(_) => {
					log::info!("Connection to {client_addr} closed");
				}
				Err(e) => {
					log::error!("Connection to {client_addr} closed with error: {e}");
				}
			}
		});
	}
}

/// Creates or asserts the existence of the proper directory structure
fn validate_dirs(config: &Config) -> Result<(), ServerError> {
	let dirs_to_validate = vec![config.data_path.clone(), config.db_path()];
	for dir_path in dirs_to_validate.into_iter() {
		if !dir_path.exists() {
			log::warn!("Creating directory: {}", dir_path.display());
			fs::create_dir(dir_path)?;
		} else if !dir_path.is_dir() {
			return Err(ServerError::Config(format!(
				"Expected directory at \"{}\", found file",
				dir_path.display()
			)));
		}
	}
	log::debug!("Directory structure validated");
	Ok(())
}
