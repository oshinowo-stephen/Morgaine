use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric; 

pub fn generate_salt() -> String {
	let mut rng = thread_rng();

	let chars: String = iter::repeat(())
		.map(|()| rng.sample(Alphanumeric))
		.map(char::from)
		.take(16)
		.collect();

	chars
}