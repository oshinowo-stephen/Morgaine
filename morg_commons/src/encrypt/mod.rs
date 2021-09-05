pub mod hasher;
pub mod random;

use super::utils::file_manager;

use aead::{Aead, NewAead};
use aes::{aead, Aes256GcmSiv, Key, Nonce};

use std::fs::File;
use std::path::Path;

pub use random::*;

pub struct CipherBlock {
	pub nonce: Nonce,
	pub text: Vec<u8>,
	pub cipher: Aes256GcmSiv,
}

#[derive(Debug)]
pub enum CipherError {
	Decryption(String),
	Encryption(String),
}

pub type Result<T> = std::result::Result<T, CipherError>;

pub fn create_encrypt_file(key: &[u8], path: &Path, incoming_data: &[u8]) -> Result<File> {
	let sig = path.file_name().unwrap().to_str();

	if let Ok(block) = encrypt_block(sig.unwrap(), key, incoming_data) {
		let file = file_manager::create_file(path, Some(block.text.as_slice()));

		Ok(file.unwrap())
	} else {
		Err(CipherError::Encryption(format!("Unable to encrypt incoming data.")))
	}

}

pub fn encrypt_block(sig: &str, k: &[u8], msg: &[u8]) -> Result<CipherBlock> {
	let incoming_key = digest(k);

	let key = Key::from_slice(incoming_key.as_ref());
	let cipher = Aes256GcmSiv::new(key);

	let supplied_nonce = format!("{}.{:?}.{}", sig, k, generate_salt());
	let incoming_nonce = digest(supplied_nonce.as_bytes());
	let nonce = Nonce::from_slice(incoming_nonce.as_ref());

	match cipher.encrypt(&nonce, msg) {
		Ok(text) => Ok(CipherBlock {
			text,
			cipher,
			nonce: nonce.clone(),
		}),
		Err(error) => Err(CipherError::Encryption(format!(
			"An encryption error was noticed: {:#?}",
			error
		))),
	}
}

pub fn decrypt_block<'d>(block: CipherBlock) -> Result<Vec<u8>> {
	match block.cipher.decrypt(&block.nonce, block.text.as_slice()) {
		Ok(result) => {
			let block = result.clone();

			Ok(block)
		},
		Err(error) => Err(CipherError::Decryption(format!(
			"An decryption error was noticed: {:#?}",
			error
		))),
	}
}
