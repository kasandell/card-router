use crate::api_error::ApiError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager,CustomizeConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::time::Duration;
use lazy_static::lazy_static;
use r2d2::Error;

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
        let mut builder = r2d2::Pool::builder();
        if cfg!(test) {
            warn!("Running test pool");
            builder = builder.connection_customizer(Box::new(TestConnectionCustomizer));
        }
        info!("Initializing connnection pool with {} connections", pool_size);
        builder.max_size(pool_size).build(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get db connection");
    run_migration(&mut conn);
    info!("Initialized DB");
}

pub fn connection() -> Result<DbConnection, Error> {
    POOL.get()
        //.map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}

#[derive(Debug, Clone, Copy)]
pub struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: diesel::Connection,
{
    fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        conn.begin_test_transaction()
            .expect("Failed to start test transaction");
        Ok(())
    }
}