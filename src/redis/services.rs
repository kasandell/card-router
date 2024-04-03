use async_trait::async_trait;
use mockall::automock;
use mockall::predicate::*;
use mockall::concretize;
use serde::{Deserialize, Serialize};
use crate::redis::error::RedisError;
use crate::redis::key::StableRedisKey;
use redis::{AsyncCommands, Client, FromRedisValue, ToRedisArgs};
use std::sync::Arc;
use std::marker::{Send, Sync};
use chrono::Duration;
use serde::de::DeserializeOwned;
use crate::redis::client::get_connection;

/// Trait Representable for any Redis Service to implement
#[async_trait(?Send)]
pub trait RedisServiceTrait {
    async fn get<K, T>(self: Arc<Self>, key: &K) -> Result<T, RedisError>
       where T: DeserializeOwned, K: StableRedisKey;
    async fn get_primitive<K, T>(self: Arc<Self>, key: &K) -> Result<T, RedisError>
        where T: FromRedisValue, K: StableRedisKey;
    async fn set<K, T>(self: Arc<Self>, key: &K, val: &T) -> Result<(), RedisError>
        where T: Serialize, K: StableRedisKey;
    async fn set_primitive<K, T>(self: Arc<Self>, key: &K, val: T) -> Result<(), RedisError>
        where T: ToRedisArgs + Send + Sync, K: StableRedisKey;
    async fn expire_in<K>(self: Arc<Self>, key: &K, time: Duration) -> Result<(), RedisError>
        where K: StableRedisKey;
    async fn expire_now<K>(self: Arc<Self>, key: &K) -> Result<(), RedisError>
        where K: StableRedisKey;
}

/// Standard Redis Service
pub struct RedisService {

}

impl RedisService {
    /// Instantiate a new Redis service
    //#[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl RedisServiceTrait for RedisService {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    async fn get<K, T>(self: Arc<Self>, key: &K) -> Result<T, RedisError> where T: DeserializeOwned, K: StableRedisKey {
        let mut conn = get_connection().await?;
        let result: String = conn.get(key.to_key()).await?;
        let consumed_result: &str = result.as_str();
        let serialized: T = serde_json::from_str(consumed_result)?;
        Ok(serialized)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    async fn get_primitive<K, T>(self: Arc<Self>, key: &K) -> Result<T, RedisError> where T: FromRedisValue, K: StableRedisKey {
        let mut conn = get_connection().await?;
        let result: T = conn.get(key.to_key()).await?;
        Ok(result)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    async fn set<K, T>(self: Arc<Self>, key: &K, val: &T) -> Result<(), RedisError> where T: Serialize, K: StableRedisKey {
        let mut conn = get_connection().await?;

        let serialized = serde_json::to_string(&val)?;
        conn.set(key.to_key(), serialized).await?;
        Ok(())
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    async fn set_primitive<K, T>(self: Arc<Self>, key: &K, val: T) -> Result<(), RedisError> where T: ToRedisArgs + Send + Sync, K: StableRedisKey {
        let mut conn = get_connection().await?;
        conn.set(key.to_key(), val).await?;
        Ok(())
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    async fn expire_in<K>(self: Arc<Self>, key: &K, time: Duration) -> Result<(), RedisError>
        where K: StableRedisKey {
        let duration = time.num_seconds();
        let mut conn = get_connection().await?;
        conn.expire(key.to_key(), duration).await?;
        Ok(())
    }
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    async fn expire_now<K>(self: Arc<Self>, key: &K) -> Result<(), RedisError>
        where K: StableRedisKey {
        self.clone().expire_in(key, Duration::nanoseconds(0)).await
    }
}



#[cfg(test)]
mod test {
    use std::sync::Arc;
    use actix_web::test;
    use diesel::row::NamedRow;
    use crate::redis::services::RedisService;
    use crate::redis::services::RedisServiceTrait;
    use crate::redis::error::RedisError;
    use crate::test_helper::redis::{TestStruct, TestKey};



    #[test]
    async fn test_get() {
        crate::test_helper::general::init();
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let test_val = TestStruct::new();
        let key = TestKey::new();
        svc.clone().set::<_, _>(&key, &test_val).await.expect("should be ok");
        let val = svc.clone().get::<_, TestStruct>(&key).await.expect("should give val");
        assert_eq!(val, test_val);
    }



    #[test]
    async fn test_get_not_found() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let error = svc.clone().get::<_, TestStruct>(&TestKey::new()).await.expect_err("shouldn't find");
        assert_eq!(RedisError::NotFound("test".into()), error);
    }

    #[test]
    async fn test_get_primitive() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let test_val = 1;
        let key = TestKey::new();
        svc.clone().set_primitive::<_, _>(&key, test_val).await;
        let mut val = svc.clone().get_primitive::<_, i32>(&key).await.expect("should give val");
        assert_eq!(val, test_val);
    }

    #[test]
    async fn test_get_primitive_not_found() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let error = svc.clone().get_primitive::<_, i32>(&TestKey::new()).await.expect_err("shouldn't find");
        assert_eq!(RedisError::NotFound("test".into()), error);
    }

    #[test]
    async fn test_set() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let key = TestKey::new();
        svc.clone().set::<_, _>(&key, &TestStruct::new()).await.expect("should be ok");
    }

    #[test]
    async fn test_set_primitive() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let test_val = 1;
        let key = TestKey::new();
        svc.clone().set_primitive::<_, _>(&key, test_val).await.expect("should be ok");
    }


    #[test]
    async fn test_set_primitive_get_struct() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let test_val = 1;
        let key = TestKey::new();
        svc.clone().set_primitive::<_, _>(&key, test_val).await.expect("no error");
        let error = svc.clone().get::<_, TestStruct>(&key).await.expect_err("shouldn't find");
        assert_eq!(RedisError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_set_struct_get_primitive() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let test_val = TestStruct::new();
        let key = TestKey::new();
        svc.clone().set::<_, _>(&key, &test_val).await.expect("no error");
        let error = svc.clone().get::<_, i32>(&key).await.expect_err("shouldn't find");
        assert_eq!(RedisError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_expire_in() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let test_val = 1;
        let key = TestKey::new();
        svc.clone().set_primitive::<_, _>(&key, test_val).await;
        let mut val = svc.clone().get_primitive::<_, i32>(&key).await.expect("should give val");
        assert_eq!(val, test_val);
        let duration = chrono::Duration::try_seconds(1).expect("Should create delta");
        svc.clone().expire_in::<_>(&key, duration).await;
        val = svc.clone().get_primitive::<_, i32>(&key).await.expect("should give val");
        assert_eq!(val, test_val);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let error = svc.clone().get_primitive::<_, i32>(&key).await.expect_err("should not find");
        assert_eq!(RedisError::NotFound("test".into()), error);
    }

    #[test]
    async fn test_expire_now() {
        crate::test_helper::general::init();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let test_val = 1;
        let key = TestKey::new();
        svc.clone().set_primitive::<_, _>(&key, test_val).await;
        let val = svc.clone().get_primitive::<_, i32>(&key).await.expect("should give val");
        assert_eq!(val, test_val);
        svc.clone().expire_now::<_>(&key).await;
        let error = svc.clone().get_primitive::<_, i32>(&key).await.expect_err("should not find");
        assert_eq!(RedisError::NotFound("test".into()), error);
    }
}