use crate::api_error::ApiError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::prelude::*;
use std::env;
use lazy_static::lazy_static;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
type DB = diesel::pg::Pg;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


fn run_migration(conn: &mut impl MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool_size = match cfg!(test) {
            true => 1,
            false => 10,
        };
        r2d2::Builder::new().max_size(pool_size).build(manager).expect("Failed to create db pool")
        //Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get db connection");
    if cfg!(test) {
        conn.begin_test_transaction().expect("Failed to start transaction");
    }
    run_migration(&mut conn);
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}