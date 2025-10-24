use clap::{Arg, Command, value_parser};
use lildb_client::LildbSession;

fn main() {
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

	let connection = LildbSession::new((host_address, host_port)).unwrap_or_else(|e| {
		eprintln!("Failed to connect to server: {e}");
		std::process::exit(1)
	});

	println!("connected");

	std::thread::sleep(std::time::Duration::from_secs_f32(4.0));

	drop(connection);

	std::process::exit(0);
}
