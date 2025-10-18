use log::{LevelFilter, Log, Metadata, Record};

/// Quick and dirty custom logging backend
///
/// TODO improve this
struct Logger;

impl Log for Logger {
	fn enabled(&self, _metadata: &Metadata) -> bool {
		true
	}

	fn log(&self, record: &Record) {
		println!("{:<5} - {}", record.level(), record.args());
	}

	fn flush(&self) {}
}

pub fn initialize() {
	log::set_logger(&Logger).expect("msg");
	log::set_max_level(LevelFilter::Trace);
}
