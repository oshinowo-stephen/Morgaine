extern crate morg_commons;

#[test]
fn grab_defaults() {
	let conf = morg_commons::config::load()
		.expect("unable to load config");

	assert_eq!(conf.server.port, 7890)
}
