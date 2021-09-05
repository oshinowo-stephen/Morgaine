use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};
use std::path::Path;

pub fn create_file(path: &Path, initial_data: Option<&[u8]>) -> Result<File> {
	match OpenOptions::new().create(true).open(path) {
		Ok(mut f) => if initial_data.is_some() {
			f.write(initial_data.unwrap());

			Ok(f)
		} else {
			Ok(f)
		}, 
		Err(error) => Err(error)
	}
}

pub fn write_data(file: &mut File, incoming_data: String, overwrite: Option<()>) -> Result<()> {
	if overwrite.is_some() {
		file.write_all(incoming_data.as_bytes());

		Ok(())
	} else {
		let to_be_written = format!("{}\n", incoming_data);

		file.write(to_be_written.as_bytes()).unwrap();

		Ok(())
	}
}

pub fn read_file(mut file: &File) -> Result<String> {
	let mut contents = String::new();

	file.read_to_string(&mut contents);

	Ok(contents)
}
