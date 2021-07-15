use std::error::Error;
use std::fmt::{self, Display, Formatter};
pub use super::schema::vaults;

pub type ModelAction<T> = std::result::Result<T, ModelError>;

#[derive(Debug)]
pub enum ModelError {
	Fetch(String),
	Insert(String),
	Remove(String),
}

impl Error for ModelError {}

impl Display for ModelError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			ModelError::Fetch(m) =>
				write!(f, "cannot fetch model: {}", m),			
			ModelError::Insert(m) =>
				write!(f, "cannot insert model: {}", m),
			ModelError::Remove(m) =>
				write!(f, "cannot remove model: {}", m)
		}
	}
}

pub mod masters {

}

pub mod vault {
	pub type Vaults = Vec<Vault>;

	use diesel::prelude::*;
	use diesel::pg::PgConnection;
	use super::vaults::{self, dsl::vaults as all_vaults};

	#[derive(Queryable, Deserialize)]
	pub struct Vault {
		pub id: String,
		pub title: String,
		pub master_id: String,
		pub created: String,
	}

	#[derive(Insertable)]
	#[table_name="vaults"]
	pub struct StoreVault {
		pub title: String,
		pub master_id: String,
	}

	pub fn store(vault: StoreVault, conn: &PgConnection) -> super::ModelAction<()> {
		Ok(())
	}

	pub fn all_from_master(master: &str, conn: &PgConnection) -> super::ModelAction<Vaults> {
		
	}

	pub fn one_from_master(master: &str, name: &str, conn: &PgConnection) -> super::ModelAction<Vault> {
		
	}
}

pub mod password {
	
}