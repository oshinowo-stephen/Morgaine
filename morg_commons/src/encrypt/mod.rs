pub mod random;
pub mod hasher;

use ring::digest::{self as dig, SHA256, Digest};
use aes::{aead, Key, Aes256GcmSiv, Nonce};
use aead::{Aead, NewAead};

pub use random::*;

pub struct CipherBlock {
	pub nonce: Nonce,
	pub text: Vec<u8>,
	pub cipher: Aes256GcmSiv 
}

pub fn encrypt_block(sig: &str, k: &[u8], msg: &[u8]) -> CipherBlock {
	let incoming_key = digest(k);

	let key = Key::from_slice(incoming_key.as_ref());
	let cipher = Aes256GcmSiv::new(key);

	let supplied_nonce = format!("{}.{:?}.{}", sig, k, random::generate_salt());
	let incoming_nonce = digest(supplied_nonce.as_bytes());
	let nonce = Nonce::from_slice(incoming_nonce.as_ref());
	
	let encrypted_text = match cipher.encrypt(&nonce, msg) {
		Ok(result) => result,
		Err(error) => panic!("An unhandled error occurred: {}", error)
	};

	CipherBlock {
		cipher,
		nonce: nonce.clone(),
		text: encrypted_text,
	}
} 

pub fn decrypt_block(block: CipherBlock) -> Vec<u8> {
	match block.cipher.decrypt(&block.nonce, block.text.as_slice()) {
		Ok(result) => result,
		Err(error) => panic!("An unknown error has occurred: {}", error)
	}
}

fn digest(incoming: &[u8]) -> Digest {
	dig::digest(&SHA256, incoming)
}

// pub type Block = aes::Aes256GcmSiv;

// pub fn initialize() -> Vec<Block> {

// }

// use aes::Aes256GcmSiv;

// pub struct Block {
// 	key: String,
// 	nonce: String,
// }

// pub type Cipher = Aes256GcmSiv;

// impl Block {
// 	pub fn new(k: &str) -> Self {
// 		Self {}
// 	}

// 	pub fn from(k: &str) -> Cipher {

// 	}
// }