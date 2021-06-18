extern crate log;
extern crate dotenv;
extern crate chrono;
extern crate colored;

pub mod logg;
pub mod menv;

#[cfg(test)]
mod tests {
	use super::logg::files;

	#[test]
	fn log_create_file() {
		assert!(files::create_log_file().is_ok())
	}

	#[test]
	fn log_append_file () {
		let mut file = files::create_log_file()
			.expect("cannot generate log file.");

		assert!(files::append_log_file(&mut file, "TEST APPEND!").is_ok())
	}
}
