use std::env;
use bb8::{Pool, PooledConnection, RunError};
use tokio::sync::OnceCell;
use redis::{Client, RedisResult};
use redis::aio::MultiplexedConnection;
use crate::environment::ENVIRONMENT;
use bb8_redis::{
    bb8,
    redis::{cmd, AsyncCommands},
    RedisConnectionManager
};
use redis::RedisError;

/// The global redis connection pool
static POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

/// Get a connection from the redis pool
/// This wrapping method allows us to hide away the actual pool
#[cfg_attr(feature="trace-detail", tracing::instrument)]
pub async fn get_connection<'a>() -> Result<PooledConnection<'a, RedisConnectionManager>, RunError<RedisError>> {
    POOL
        .get_or_init(init_pool)
        .await
        .get()
        .await
}

/// Initialize the redis pool with specified connections
#[cfg_attr(feature="trace-detail", tracing::instrument)]
pub async fn init_pool() -> Pool<RedisConnectionManager>{
    let pool_size = match cfg!(test) {
        true => 1,
        false => 50,
    };
    let mut builder = Pool::builder();
    let manager = RedisConnectionManager::new(ENVIRONMENT.redis_url.clone()).unwrap();
    builder.max_size(pool_size).build(manager).await.expect("Failed to create db pool")
}