extern crate morg_commons;
extern crate async_std;
extern crate futures;
extern crate asynv;

use morg_commons::config::load as load_config;

type AsyncResult = std::io::Result<()>;
 
#[async_std::test]
async fn load_from_env() -> AsyncResult {
	// setup environment.
	asynv::set("MORG_ADDR", "127.0.0.1:7880").await;

	// load from env.
	let conf = load_config(None);

	Ok(assert_eq!(&conf.address, "127.0.0.1:7880"))
}

#[async_std::test]
async fn load_from_file() -> AsyncResult {
	// clear vars if any.
	asynv::rm("MORG_ADDR").await;

	// load from path.
	let path = Some("__test__/data/mock_config.yml");

	let conf = load_config(path);

	Ok(assert_eq!(&conf.address, "127.0.0.1:6880"))
}

#[async_std::test]
async fn isnt_met_with_env() -> AsyncResult {
	// setup environment.
	asynv::set("MORG_ADDR", "127.0.0.1:7880").await;

	// load from files
	let p = Some("__tests__/data/mock_config");
	let config = load_config(p);

	// env loaded in from: `load_from_env` test.
	assert_eq!(&config.address, "127.0.0.1:7880");
	Ok(assert_eq!(&config.db_client, "sqlite"))
}

#[async_std::test]
async fn load_defaults() -> AsyncResult {
	// clear current environment
	asynv::rm("MORG_ADDR").await;

	let config = load_config(None);

	Ok(assert_eq!(&config.address, "127.0.0.1:6880"))
}