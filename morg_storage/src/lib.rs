#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;
extern crate uuid;
extern crate refinery;
extern crate morg_commons;

pub mod schema;
// pub mod models;
// pub mod functions;
// pub mod migrations;

use morg_commons::menv;
use morg_commons::logg;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use std::env::var as envar;

type Conn = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn setup() -> diesel::ConnectionResult<Conn> {
	menv::load(); 
	logg::setup();

	let database_url = envar("MORG_DB_URL").expect("cannot load in the URL"); 

	let manager = ConnectionManager::<PgConnection>::new(&database_url);
	info!("Connecting and constructing to: {:#?}.", &database_url);

	Ok(r2d2::Pool::new(manager).expect("cannot create manager."))
}