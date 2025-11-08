mod disk;
mod objects;
mod record;

use std::{fs, path::PathBuf};

use crate::*;
use disk::DiskManager;

pub struct LilDbConnection {
	opts: LilDbOpts,
	disk: DiskManager,
}
impl LilDbConnection {
	pub fn open_db(path: PathBuf, opts: LilDbOpts) -> Result<LilDbConnection> {
		let f = fs::OpenOptions::new()
			.read(true)
			.write(true)
			.create(opts.create)
			.open(path)?;
		let disk = DiskManager::new(f)?;
		Ok(LilDbConnection { opts, disk })
	}
}
