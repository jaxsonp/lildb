mod logging;

use log;

pub fn run() -> i32 {
	logging::initialize();

	log::info!("Logging initialized");

	return 0;
}
