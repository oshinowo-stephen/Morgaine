// extern crate morg_commons;
// extern crate async_std;
// extern crate futures;
// extern crate asynv;

// use std::path::Path;
// use morg_commons::config::load as load_config;

// type AsyncResult = std::io::Result<()>;
 
// #[async_std::test]
// async fn load_from_env() -> AsyncResult {
// 	// setup environment.
// 	asynv::set("MORG_ADDR", "127.0.0.1:7880").await;
// 	asynv::set("MORG_DB_ADDR", "127.0.0.1:5432").await;

// 	// load from env.
// 	let conf = load_config(Path::new("")).await.unwrap();
// 	asynv::rm("MORG_DB_ADDR").await;

// 	Ok(assert_eq!(&conf.address.unwrap(), "127.0.0.1:7880"))
// }

// #[async_std::test]
// async fn load_from_file() -> AsyncResult {
// 	// clear vars if any.
// 	asynv::rm("MORG_ADDR").await;

// 	// load from path.
// 	let p = "__tests__/data/mock_config.yml";
// 	let config = load_config(Path::new(p)).await.unwrap();
// 	asynv::rm("MORG_DB_ADDR").await;

// 	Ok(assert_eq!(&config.address.unwrap(), "127.0.0.1:6880"))
// }

// #[async_std::test]
// async fn isnt_met_with_env() -> AsyncResult {
// 	// setup environment.
// 	asynv::set("MORG_ADDR", "127.0.0.1:7880").await;

// 	// load from files
// 	let p = "__tests__/data/mock_config.yml";
// 	let config = load_config(Path::new(p)).await.unwrap();
// 	asynv::rm("MORG_DB_ADDR").await;

// 	// env loaded in from: `load_from_env` test.
// 	assert_eq!(&config.address.unwrap(), "127.0.0.1:7880");
// 	Ok(assert_eq!(&config.database.unwrap().password, "something12"))
// }

// #[async_std::test]
// async fn load_defaults() -> AsyncResult {
// 	// clear current environment
// 	asynv::rm("MORG_ADDR").await;
// 	asynv::set("MORG_DB_ADDR", "127.0.0.1:5432").await;

// 	let config = load_config(Path::new("")).await.unwrap();
// 	asynv::rm("MORG_DB_ADDR").await;

// 	Ok(assert_eq!(&config.address.unwrap(), "127.0.0.1:6880"))
// }