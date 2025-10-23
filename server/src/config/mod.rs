#[cfg(test)]
mod tests;

use std::{
	collections::HashMap,
	io::Read,
	net::{IpAddr, Ipv6Addr},
	path::{Path, PathBuf},
	str::FromStr,
	sync::{Arc, OnceLock},
};

use crate::*;

static GLOBAL_CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

pub fn initialize_global_config(c: Config) -> Result<(), ServerError> {
	GLOBAL_CONFIG.set(Arc::new(c)).map_err(|_c| {
		ServerError::Internal("Attempted to double initialize global config".to_string())
	})
}

pub fn config() -> Result<Arc<Config>, ServerError> {
	GLOBAL_CONFIG
		.get()
		.ok_or(ServerError::Internal(
			"Attempted to access global config before initialization".to_string(),
		))
		.map(|config| config.clone())
}

/// Config for the daemon
///
/// More stuff will go here eventually
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Config {
	/// Path to data directory
	pub data_path: PathBuf,
	/// Address to receive requests on
	pub listen_addr: IpAddr,
	/// Port to receive requests on
	pub listen_port: u16,
}

impl Config {
	/// Read config from file
	pub fn from_file(path: &Path) -> Result<Config, ServerError> {
		if !path.exists() {
			return Err(ServerError::Config(format!(
				"\"{}\" does not exist",
				path.display()
			)));
		} else if !path.is_file() {
			return Err(ServerError::Config(format!(
				"\"{}\" is not a file",
				path.display()
			)));
		}
		log::debug!("Reading config from file: {}", path.display());

		let mut content = String::new();
		fs::OpenOptions::new()
			.read(true)
			.open(path)?
			.read_to_string(&mut content)?;
		let lines = content.split('\n');

		// read pairs into hashmap
		let mut items: HashMap<&str, &str> = HashMap::new();
		for (line_num, line) in lines.enumerate() {
			if line
				.chars()
				.next()
				.is_none_or(|first_char| first_char == '#')
			{
				continue;
			}
			let (key, value) = line.split_once('=').ok_or(ServerError::Config(format!(
				"Line {line_num} malformed: \"{line}\""
			)))?;
			items.insert(key, value);
		}

		let mut config = Config::default();

		if let Some(path) = items.remove("DATA_PATH") {
			config.data_path = PathBuf::from(path);
			log::debug!(" - Data directory: {}", config.data_path.display());
		}

		if let Some(address_str) = items.remove("LISTEN_ADDR") {
			config.listen_addr = IpAddr::from_str(address_str).map_err(|_| {
				ServerError::Config(format!(
					"Failed to parse listen address \"{}\" in config file",
					address_str
				))
			})?;
			log::debug!(" - Listen address: {}", config.listen_addr);
		}

		if let Some(port_str) = items.remove("LISTEN_PORT") {
			config.listen_port = port_str.parse::<u16>().map_err(|_| {
				ServerError::Config(format!(
					"Failed to parse listen port \"{}\" in config file",
					port_str
				))
			})?;
			log::debug!(" - Listen port: {}", config.listen_port);
		}

		// leftover values
		if let Some((key, _value)) = items.iter().next() {
			return Err(ServerError::Config(format!(
				"Unrecognized option: \"{}\"",
				key
			)));
		}
		return Ok(config);
	}

	/// Directory for databases
	pub fn db_path(&self) -> PathBuf {
		self.data_path.join("databases")
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			data_path: Path::new("./lildb-data/").to_path_buf(),
			listen_addr: IpAddr::V6(Ipv6Addr::UNSPECIFIED),
			listen_port: 11108,
		}
	}
}
