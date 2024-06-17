use std::time::Duration;
use async_trait::async_trait;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use diesel::{Connection, PgConnection};
use diesel::prelude::*;
use diesel_async::pooled_connection::{bb8::{RunError, Pool}, AsyncDieselConnectionManager};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection as SyncCustomizeConnection;
use bb8::{PooledConnection, CustomizeConnection};
use secrecy::ExposeSecret;
use tokio::sync::OnceCell;
use tonic::codegen::tokio_stream::StreamExt;
use crate::configuration::configuration::get_global_configuration;

pub type ConnManage = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbConnection<'a> = PooledConnection<'a, ConnManage>;
type DB = diesel::pg::Pg;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


fn run_migration(conn: &mut impl MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

static POOL: OnceCell<Pool<AsyncPgConnection>> = OnceCell::const_new();
static SYNC_POOL: OnceCell<r2d2::Pool<ConnectionManager<PgConnection>>> = OnceCell::const_new();

pub async fn init_db() -> Pool<AsyncPgConnection>{
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(get_database_url().await);
    let config = &get_global_configuration().await.database;
    let pool_size = config.pool_size;
    let mut builder = Pool::builder();
     if cfg!(test) {
         tracing::warn!("Running test pool");
         builder = builder.connection_customizer(Box::new(TestConnectionCustomizer));
     }
    tracing::info!("Initializing connection pool with {} connections", pool_size);
    builder.max_size(pool_size).connection_timeout(Duration::from_secs(2)).build(manager).await.expect("Failed to create db pool")
}

pub async fn init_sync_db() -> r2d2::Pool<ConnectionManager<PgConnection>> {

    let manager = ConnectionManager::<PgConnection>::new(get_database_url().await);
    let pool_size = 1;
    let mut builder = r2d2::Pool::builder();
    if cfg!(test) {
        tracing::warn!("Running test pool");
        builder = builder.connection_customizer(Box::new(SyncTestConnectionCustomizer));
    }
    tracing::info!("Initializing connnection pool with {} connections", pool_size);
    builder.max_size(pool_size).connection_timeout(Duration::from_secs(2)).build(manager).expect("Failed to create db pool")
}

pub async fn init() {
    tracing::info!("Initializing DB");
    POOL.get_or_init(init_db).await;
    let mut conn = SYNC_POOL.get_or_init(init_sync_db).await.get().expect("need sync connection");
    run_migration(&mut conn);
    tracing::info!("Initialized DB");
}

pub async fn get_database_url() -> String {
    let config = &get_global_configuration().await.database;
    // TODO: extract and test this
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &config.username,
        &config.password.expose_secret(),
        &config.host,
        &config.port,
        &config.database_name
    );

    url
}

pub type ConnResult<'a> = Result<PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>, RunError>;
#[cfg_attr(feature="trace-detail", tracing::instrument)]
pub async fn connection<'a>() -> ConnResult<'a> {
    POOL.get_or_init(init_db).await.get().await
        //.map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}



#[derive(Debug, Clone, Copy)]
pub struct TestConnectionCustomizer;

#[async_trait]
impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: AsyncConnection + 'static,
    E: 'static
{
    async fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        conn.begin_test_transaction().await
            .expect("Failed to start test ledger");
        Ok(())
    }
}


#[derive(Debug, Clone, Copy)]
pub struct SyncTestConnectionCustomizer;
impl<C, E> SyncCustomizeConnection<C, E> for SyncTestConnectionCustomizer
where
    C: Connection
{
    fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        conn.begin_test_transaction()
            .expect("Failed to start test ledger");
        Ok(())
    }
}