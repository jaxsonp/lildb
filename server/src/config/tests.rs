use std::{io::Write, net::Ipv4Addr};

use tempfile::NamedTempFile;

use super::*;

#[test]
fn config_file() {
	let mut config_file = NamedTempFile::new().unwrap();
	write!(
		config_file,
		r#"
DATA_PATH=/peepee/poopoo

# comment
LISTEN_ADDR=12.34.56.78
LISTEN_PORT=1984

"#
	)
	.unwrap();
	let config_path = config_file.into_temp_path();
	let config = Config::from_file(&config_path).unwrap();

	assert_eq!(
		config,
		Config {
			data_path: PathBuf::new().join("/peepee").join("poopoo"),
			listen_addr: IpAddr::V4(Ipv4Addr::new(12, 34, 56, 78)),
			listen_port: 1984,
		}
	)
}

#[test]
fn empty_config_file() {
	let config_path = NamedTempFile::new().unwrap().into_temp_path();
	let config = Config::from_file(&config_path).unwrap();

	assert_eq!(config, Config::default());
}

#[test]
fn invalid_key() {
	let mut config_file = NamedTempFile::new().unwrap();
	write!(
		config_file,
		r#"
FAKE=1234
"#
	)
	.unwrap();
	assert!(matches!(
		Config::from_file(&config_file.into_temp_path()),
		Err(ServerError::Config(_))
	));
}

#[test]
fn invalid_values() {
	let mut config_file = NamedTempFile::new().unwrap();
	write!(
		config_file,
		r#"
LISTEN_PORT=abcdefg
"#
	)
	.unwrap();
	assert!(matches!(
		Config::from_file(&config_file.into_temp_path()),
		Err(ServerError::Config(_))
	));

	config_file = NamedTempFile::new().unwrap();
	write!(
		config_file,
		r#"
LISTEN_ADDRESS=256.256.256.256
"#
	)
	.unwrap();
	assert!(matches!(
		Config::from_file(&config_file.into_temp_path()),
		Err(ServerError::Config(_))
	));
}
