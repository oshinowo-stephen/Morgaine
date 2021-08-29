extern crate morg_commons;

use morg_commons::encrypt;

#[test]
fn test_hasher() {
	let hash = encrypt::hasher::hash("uwu_kawaii177013"); // A Password I use on a daily basis

	assert!(encrypt::hasher::compare(&hash, "uwu_kawaii177013"));
}

#[test]
fn test_random_len() {
	assert!(encrypt::random::generate_salt().len() == 16)
}