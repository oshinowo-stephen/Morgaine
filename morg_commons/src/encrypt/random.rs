use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

use ring::digest::{self as dig, Digest, SHA256};

pub fn digest(incoming: &[u8]) -> Digest {
	dig::digest(&SHA256, incoming)
}

pub fn generate_salt() -> String {
	let mut rng = thread_rng();

	let chars: String = iter::repeat(())
		.map(|()| rng.sample(Alphanumeric))
		.map(char::from)
		.take(16)
		.collect();

	chars
}
