use diesel::pg::PgConnection;

pub fn create_pg_conn() -> diesel::r2d2::PooledConnection<PgConnection> {}