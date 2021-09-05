use crate::utils::file_manager;
use chrono::Local;

use std::fs::File;
use std::io;
use std::path::Path;

pub fn create_log_file() -> io::Result<File> {
	let timestamp = Local::now().format("%Y:%m:%d");

	let file_name = format!("logs/{}.log", timestamp);

	file_manager::create_file(Path::new(&file_name), None)
}

pub fn append_log_file(f: &mut File, append: &str) -> io::Result<()> {
	file_manager::write_data(f, append.to_owned(), None)
}
