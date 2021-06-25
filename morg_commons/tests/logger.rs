extern crate morg_commons;

use morg_commons::logg::files;

#[test]
fn logg_create_file() {
	assert!(files::create_log_file().is_ok())
}

#[test]
fn logg_append_file() {
	if let Ok(mut file) = files::create_log_file() {
		assert!(files::append_log_file(&mut file, "Hello, World!\n").is_ok())
	} else {
		panic!("cannot append to file, file doesn't exist.")
	}
}