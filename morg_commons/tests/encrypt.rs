extern crate morg_commons;
extern crate async_std;

use std::io;

use morg_commons::encrypt;

#[async_std::test]
async fn test_hasher() -> io::Result<()> {
	let salt = encrypt::random::generate_salt(); // Generates a random 16 character salt for me.

	let hash = encrypt::hasher::hash(b"uwu_kawaii177013", salt.as_bytes(), b""); // A Password I use on a daily basis

	assert!(encrypt::hasher::comp(b"uwu_kawaii177013", hash.as_slice()));

	Ok(())
}

#[async_std::test]
async fn test_random_len() -> io::Result<()> {
	assert!(encrypt::random::generate_salt().len() == 16);

	Ok(())
}