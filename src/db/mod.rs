mod disk_mgr;

use std::{
	fs::{self, File},
	io,
	path::PathBuf,
};

use crate::*;
use disk_mgr::DiskManager;

pub struct DbConnection {
	disk: DiskManager,
}
impl DbConnection {
	pub fn open_db(path: PathBuf) -> Result<DbConnection> {
		let f = fs::OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(false)
			.open(path)?;
		let disk = DiskManager::new(f)?;
		Ok(DbConnection { disk })
	}

	pub fn create_db(path: PathBuf) -> Result<DbConnection> {
		let f = fs::OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			.open(path)?;
		let disk = DiskManager::init_db(f)?;
		Ok(DbConnection { disk })
	}
}
