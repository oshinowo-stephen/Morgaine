extern crate morg_commons;

use morg_commons::encrypt;

#[test]
fn test_encrypt_block() {
	let encryption_sig = "some_user_id0115";
	let encryption_key = "super_secret_key123";
	let encryption_string = "anime girls are lovely";

	let encryption = encrypt::encrypt_block(
		encryption_sig, 
		encryption_key.as_bytes(),
		encryption_string.as_bytes()
	);
	
	let block = encryption.unwrap();

	let decryption = encrypt::decrypt_block(block);

	dbg!(&std::str::from_utf8(&decryption.unwrap()).unwrap());

	assert_eq!(2 + 2, 4)
}

#[test]
fn test_hasher() {
	let hash = encrypt::hasher::hash("uwu_kawaii177013"); // A Password I use on a daily basis

	assert!(encrypt::hasher::compare(&hash, "uwu_kawaii177013"));
}

#[test]
fn test_random_len() {
	assert!(encrypt::random::generate_salt().len() == 16)
}
