use std::str::from_utf8 as u8_to_string;
use scrypt::{scrypt as scrypt_hash, Params, Scrypt};
use scrypt::password_hash::{PasswordHash, PasswordVerifier};

pub fn comp(incoming: &[u8], hash: &[u8]) -> bool {
	let stringed_hash = u8_to_string(hash).unwrap();
	let phrase_hash = PasswordHash::new(stringed_hash).unwrap();
	Scrypt.verify_password(incoming, &phrase_hash).is_ok()
}

pub fn hash(incoming: &[u8], salt: &[u8], pepper: &[u8]) -> Vec<u8> {
	let mut outcome: Vec<u8> = Vec::new();
	let incoming_salt = format!("{:?}.{:?}", salt, pepper);
	let outgoing_salt = super::digest(incoming_salt.as_bytes());
	let params = Params::recommended();

	scrypt_hash(incoming, outgoing_salt.as_ref(), &params, &mut outcome)
		.expect("hasher ran into an issue: cannot hash this password.");

	outcome
}