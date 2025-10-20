use std::{env, process::exit};

fn main() {
	// get config file path first from args, then from the environment vars
	let config_path = env::args()
		.skip(1)
		.next()
		.or(env::var("LILDB_CONFIG_PATH").ok());

	// run daemon
	let exit_code: i32 = match lildb::run_daemon(config_path) {
		Ok(_) => 0,
		Err(e) => {
			log::error!("{e}");
			1
		}
	};
	log::info!("Daemon exiting: {exit_code}");
	exit(exit_code);
}
