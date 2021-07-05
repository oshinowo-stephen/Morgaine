extern crate morg_commons;
extern crate async_std;

use std::io;

use morg_commons::logg::files;

#[async_std::test]
async fn logg_create_file() -> io::Result<()> {
	assert!(files::create_log_file().is_ok());

	Ok(())
}

#[async_std::test]
async fn logg_append_file() -> io::Result<()> {
	if let Ok(mut file) = files::create_log_file() {
		assert!(files::append_log_file(&mut file, "Hello, World!\n").is_ok())
	} else {
		panic!("cannot append to file, file doesn't exist.")
	}

	Ok(())
}