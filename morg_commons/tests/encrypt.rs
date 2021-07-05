extern crate morg_commons;
extern crate async_std;

use std::io;

use morg_commons::encrypt;

#[async_std::test]
async fn test_hasher() -> io::Result<()> {
	let hash = encrypt::hasher::hash("uwu_kawaii177013"); // A Password I use on a daily basis

	assert!(encrypt::hasher::compare(&hash, "uwu_kawaii177013"));

	Ok(())
}

#[async_std::test]
async fn test_random_len() -> io::Result<()> {
	assert!(encrypt::random::generate_salt().len() == 16);

	Ok(())
}