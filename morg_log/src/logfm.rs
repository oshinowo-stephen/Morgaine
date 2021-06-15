use chrono::Local;

use std::io::{self, Write};
use std::fs::{self, File, OpenOptions};

pub fn create_log_file () -> io::Result<File> {
	let timestamp = Local::now().format("%Y:%m:%d");

	let file_name = format!("logs/{}.log", timestamp);

	match OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open(&file_name)
	{
		Ok(file) => Ok(file),
		Err(error) => match error.kind() {
			io::ErrorKind::NotFound => {
				fs::create_dir("logs")?;

				create_log_file()
			},
			_ => Err(error),
		}
	}
}

pub fn append_log_file (f: &mut File, append: &str) -> io::Result<()> {
	if let Err(err) = writeln!(f, "{}", append) {
		Err(err)
	} else {
		Ok(())
	}
}
