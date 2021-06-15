use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

const SECRETS: &str = "/run/secrets/";

pub fn load() -> dotenv::Result<std::path::PathBuf> {
	let entries = fs::read_dir(SECRETS)
		.expect("cannot find directory")
		.map(|res| res.map(|e| e.path()))
		.collect::<Result<Vec<_>, io::Error>>()
		.expect("cannot return this value.");

	for path_buf in entries {
		let path = path_buf.as_path();

		if let Ok(contents) = read_secret(path) {
			load_to_env(path.file_name().unwrap().to_str().unwrap(), &contents)
				.expect("cannot load env.");
		}
	}

	Ok(std::path::PathBuf::new())
}

fn read_secret(p: &Path) -> io::Result<String> {
	let mut contents = String::new();

	let mut file = File::open(p)?;

	file.read_to_string(&mut contents)?;

	Ok(contents)
}

fn load_to_env(name: &str, v: &str) -> io::Result<()> {
	std::env::set_var(name, v);

	Ok(())
}
