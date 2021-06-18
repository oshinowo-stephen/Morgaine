use log::{
	Log,
	Level,
	Record, 
	Metadata, 
};
use colored::*;
use chrono::Local;

use std::env;

pub struct MorgLogger;

impl Log for MorgLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		if env::var("MORGAINE_ENV").unwrap() == "production" {
			metadata.level() <= Level::Info
		} else {
			metadata.level() <= Level::Debug
		} 
	}

	fn log(&self, r: &Record) {
		let lvl_str = get_lvl_str(r.level());

		let target = if !r.target().is_empty() {
			r.target()
		} else {
			r.module_path().unwrap_or_default()
		};

		let contents = format!(
			"[{}] ({}) \"{}\" | {}",
			Local::now().format("%Y/%m/%d-%H:%M:%S"),
			lvl_str,
			target,
			r.args(),
		);

		match super::files::create_log_file() {
			Ok(mut file) => match super::files::append_log_file(&mut file, &contents) {
				Ok(_) => {},
				Err(error) => eprintln!("unable to append to current log file: {}", error)
			},
			Err(err) => eprintln!("cannot create log file, reason: {}", err)
		}

		println!("{}", contents);
	}

	fn flush(&self) {}
}


fn get_lvl_str(lvl: log::Level) -> String {
	let lvl_str = match lvl {
		log::Level::Info => lvl.to_string().cyan(),
		log::Level::Warn => lvl.to_string().yellow(),
		log::Level::Error => lvl.to_string().red(),
		log::Level::Debug => lvl.to_string().purple(),
		log::Level::Trace => lvl.to_string().normal(),
	};

	lvl_str.to_string()
}
