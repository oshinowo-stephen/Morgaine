extern crate log;
extern crate chrono;
extern crate colored;
extern crate morg_env;

pub mod logfm;
mod manager;

use std::boxed::Box;

pub fn setup() {
	log::set_max_level(log::LevelFilter::Debug);

	if let Err (error) = log::set_boxed_logger(Box::from(manager::MorgLogger)) {
		eprintln!("unable to settup morgaine logger: {}", error)
	}
}

#[cfg(test)]
mod tests {
	use super::logfm;
		
	#[test]
	fn create_log_file() {
		assert!(logfm::create_log_file().is_ok())
	}

	#[test]
	fn append_to_log() {
		let mut file = logfm::create_log_file().unwrap();

		assert!(logfm::append_log_file(&mut file, "Hello, World!").is_ok())
	}
}
