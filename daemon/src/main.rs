#[cfg(target_os = "linux")]
fn main() {
	std::process::exit(lildb_daemon::run());
}

#[cfg(not(target_os = "linux"))]
fn main() {
	compile_error!("Unsupported OS");
}
