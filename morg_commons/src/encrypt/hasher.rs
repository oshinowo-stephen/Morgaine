use rand::rngs::OsRng;
use scrypt::{
	password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
	Scrypt,
};

pub fn hash(passwd: &str) -> String {
	let salt = SaltString::generate(&mut OsRng);

	let hashed_phrase = Scrypt
		.hash_password_simple(passwd.as_bytes(), salt.as_ref())
		.expect("cannot hash password")
		.to_string();

	hashed_phrase
}

pub fn compare(hash: &str, passwd: &str) -> bool {
	let phrase_hash = PasswordHash::new(hash).unwrap();

	Scrypt
		.verify_password(passwd.as_bytes(), &phrase_hash)
		.is_ok()
}
