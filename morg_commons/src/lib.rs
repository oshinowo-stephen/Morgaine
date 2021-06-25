#[macro_use]
extern crate log;
extern crate ring;
extern crate scrypt;
extern crate dotenv;
extern crate chrono;
extern crate colored;
#[macro_use]
extern crate serde;
extern crate serde_yaml;
extern crate aes_gcm_siv as aes;

pub mod logg;
pub mod menv;
pub mod config;

#[macro_export]
macro_rules! stringify {
	($x:expr) => {
		$x.to_string()
	};
}