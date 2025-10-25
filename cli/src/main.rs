use std::{
	io::{self, BufRead},
	process::ExitCode,
};

use clap::{Arg, Command, value_parser};
use lildb_client::LildbSession;

fn main() -> ExitCode {
	// TODO remove or improve
	simple_logger::init_with_level(log::Level::Trace).unwrap();

	// CLI args
	let args = Command::new("lildb-cli")
		.version(env!("CARGO_PKG_VERSION"))
		.about("LilDB command line interface")
		.arg(
			Arg::new("host")
				.value_name("HOST")
				.help("LilDB server address")
				.default_value("localhost")
				.index(1),
		)
		.arg(
			Arg::new("port")
				.short('p')
				.long("port")
				.help("LilDB network port")
				.default_value("11108")
				.value_parser(value_parser!(u16).range(1..)),
		)
		.get_matches();

	let host_address = args
		.get_one::<String>("host")
		.expect("arg with default")
		.as_str();
	let host_port = *args.get_one::<u16>("port").expect("arg with default");
	eprintln!("Connecting to server at {host_address}:{host_port}");

	let session = match LildbSession::new((host_address, host_port)) {
		Ok(s) => s,
		Err(e) => {
			eprintln!("Error while establishing session\n{e}");
			return ExitCode::FAILURE;
		}
	};

	println!("connected");

	let stdin = io::stdin();

	loop {
		// getting input
		let query = {
			let mut stdin_handle = stdin.lock();
			let mut buf = String::new();
			match stdin_handle.read_line(&mut buf) {
				Ok(bytes_read) => {
					if bytes_read == 0 {
						// read EOF, end session
						drop(session);
						return ExitCode::SUCCESS;
					}
				}
				Err(e) => {
					eprintln!("{e}");
					return ExitCode::FAILURE;
				}
			};
			buf
		};
		eprintln!("input: {query}");
	}
}
