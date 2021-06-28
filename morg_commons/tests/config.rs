extern crate morg_commons;

use morg_commons::config::load as load_config;
use std::env::{set_var as set_env, remove_var as del_env};
 
#[test]
fn load_from_env() {
	// setup environment.
	set_env("MORG_ADDR", "127.0.0.1:7880");

	// load from env.
	let conf = load_config(None);

	assert_eq!(&conf.address, "127.0.0.1:7880")
}

#[test]
fn load_from_file() {
	// clear vars if any.
	del_env("MORG_ADDR");

	// load from path.
	let path = Some("__test__/data/mock_config.yml");

	let conf = load_config(path);

	assert_eq!(&conf.address, "127.0.0.1:6880")
}

#[test]
fn isnt_met_with_env() {
	// setup environment.
	set_env("MORG_ADDR", "127.0.0.1:7880");

	// load from files
	let p = Some("__tests__/data/mock_config");
	let config = load_config(p);

	// env loaded in from: `load_from_env` test.
	assert_eq!(&config.address, "127.0.0.1:7880");
	assert_eq!(&config.db_client, "sqlite");
}

#[test]
fn load_defaults() {
	// clear current environment
	del_env("MORG_ADDR");

	let config = load_config(None);

	assert_eq!(&config.address, "127.0.0.1:6880")
}