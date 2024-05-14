use std::env;
use bb8::{Pool, PooledConnection, RunError};
use tokio::sync::OnceCell;
use redis::{Client, RedisResult};
use redis::aio::MultiplexedConnection;
use bb8_redis::{
    bb8,
    redis::{cmd, AsyncCommands},
    RedisConnectionManager
};
use redis::RedisError;
use crate::configuration::configuration::get_global_configuration;

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
    let config = &get_global_configuration().await.redis;
    // TODO: extract and test this
    let redis_url = format!("{}:{}", config.url, config.port);
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    builder.max_size(pool_size).build(manager).await.expect("Failed to create db pool")
}