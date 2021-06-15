extern crate dotenv;

mod secrets;

use std::env;

pub fn load(secrets: bool) {
	match env::var("MORGAINE_ENV") {
		Ok(has_env) => {
			let env_store = if !secrets && has_env != "production" {
				dotenv::dotenv()
			} else {
				secrets::load()
			};

			env_store.ok();
		}
		Err(_) => {
			env::set_var("MORGAINE_ENV", "development");

			self::load(secrets);
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
