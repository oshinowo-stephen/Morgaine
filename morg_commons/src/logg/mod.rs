pub mod files;
pub mod manager;

pub fn setup() {
	let logger = Box::new(manager::MorgLogger);

	log::set_max_level(log::LevelFilter::Debug);
	if let Err(error) = log::set_boxed_logger(logger) {
		eprintln!("Unable to setup logger, reason: {}", error)
	}
}