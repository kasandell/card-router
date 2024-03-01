use async_trait::async_trait;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::environment::ENVIRONMENT;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use diesel::{Connection, PgConnection};
use diesel_async::pooled_connection::{bb8::{RunError, Pool}, AsyncDieselConnectionManager};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection as SyncCustomizeConnection;
use bb8::{PooledConnection, CustomizeConnection};
use tokio::sync::OnceCell;


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
   let db_url = ENVIRONMENT.database_url.clone();
   let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
   let pool_size = match cfg!(test) {
       true => 1,
       false => 10,
   };
   let mut builder = Pool::builder();
    if cfg!(test) {
        warn!("Running test pool");
        builder = builder.connection_customizer(Box::new(TestConnectionCustomizer));
    }
   info!("Initializing connnection pool with {} connections", pool_size);
   builder.max_size(pool_size).build(manager).await.expect("Failed to create db pool")
}

pub async fn init_sync_db() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let db_url = ENVIRONMENT.database_url.clone();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool_size = 1;
    let mut builder = r2d2::Pool::builder();
    if cfg!(test) {
        warn!("Running test pool");
        builder = builder.connection_customizer(Box::new(SyncTestConnectionCustomizer));
    }
    info!("Initializing connnection pool with {} connections", pool_size);
    builder.max_size(pool_size).build(manager).expect("Failed to create db pool")
}

pub async fn init() {
    info!("Initializing DB");
    POOL.get_or_init(init_db).await;
    let mut conn = SYNC_POOL.get_or_init(init_sync_db).await.get().expect("need sync connection");
    run_migration(&mut conn);
    info!("Initialized DB");
}

pub type ConnResult<'a> = Result<PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>, RunError>;
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
            .expect("Failed to start test transaction");
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
            .expect("Failed to start test transaction");
        Ok(())
    }
}