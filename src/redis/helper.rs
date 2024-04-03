use std::future::Future;
use std::sync::Arc;
use actix_web::web::Data;
use chrono::Duration;
use diesel::Queryable;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::error::data_error::DataError;
use crate::redis::error::RedisError;
use crate::redis::key::StableRedisKey;
use crate::redis::services::{RedisService, RedisServiceTrait};

// TODO: I don't know how to test this file in full, because I can't mock redis service
pub async fn try_redis_fallback_db<T, K, F, Fut>(
    redis_service: Arc<RedisService>,
    key: K,
    db_operation: F,
    renew_ttl: bool
) -> Result<T, DataError>
where T: Serialize + DeserializeOwned, K: StableRedisKey, F: FnOnce() -> Fut,
    Fut: Future<Output=Result<T, DataError>>
{
    let redis_response: Result<T, RedisError> = redis_service.clone().get::<_, T>(&key).await;
    match redis_response {
        Ok(val) => {
            tracing::info!("Returning {} from redis", key.to_key());
            if renew_ttl {
                expire_key_timed(redis_service.clone(), &key).await;
            }
            Ok(val)
        }
        Err(_) => {
            tracing::info!("Falling back to database operation");
            let db_result = db_operation().await?;//.await?;
            let redis_save = redis_service.clone().set::<_, T>(&key, &db_result).await;
            expire_key_timed(redis_service.clone(), &key).await;

            match redis_save {
                Ok(_) => {
                    tracing::info!("Saved in redis");
                },
                Err(e) => {
                    tracing::warn!("Error saving in redis {:?}", &e);
                }
            }
            Ok(db_result)
        }
    }
}

async fn expire_key_timed<K>(
    redis_service: Arc<RedisService>,
    key: &K
) where K: StableRedisKey {
    expire_key_timed_with_duration(redis_service.clone(), key, 120).await
}

async fn expire_key_timed_with_duration<K>(
    redis_service: Arc<RedisService>,
    key: &K,
    seconds: i64
) where K: StableRedisKey {
    if let Some(duration) = Duration::try_seconds(seconds) {
        tracing::info!("Expiring redis key in 2 minutes");
        let redis_expire = redis_service.clone().expire_in::<_>(key, duration).await;
        match redis_expire {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Error setting redis expiry: {:?}", &e);
                expire_immediate(redis_service.clone(), key).await;
            }
        }
    } else {
        expire_immediate(redis_service.clone(), key).await;
    }
}

async fn expire_immediate<K>(
    redis_service: Arc<RedisService>,
    key: &K,
) where K: StableRedisKey {
    tracing::info!("Expiring redis key immediately");
    let redis_expire = redis_service.clone().expire_now::<_>(key).await;
    match redis_expire {
        Ok(_) => {},
        Err(e) => {
            tracing::error!("Error setting redis expiry: {:?}", &e);
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use actix_web::test;
    use redis::Commands;
    use crate::error::data_error::DataError;
    use crate::redis::error::RedisError;
    use crate::redis::helper::{expire_immediate, expire_key_timed_with_duration, try_redis_fallback_db};
    use crate::redis::services::{RedisService, RedisServiceTrait};
    use crate::test_helper::redis::{TestKey, TestStruct};

    #[test]
    async fn test_finds_in_redis() {
        let key = TestKey::new();
        let val = TestStruct::new();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        svc.clone().set::<_, _>(&key, &val).await.expect("fine");
        let found: TestStruct = try_redis_fallback_db(
            svc.clone(),
            key,
            || async {return Err(DataError::Unexpected("test".into()));},
            false
        ).await.expect("should give back struct");
        assert_eq!(found.field, val.field);
    }

    #[test]
    async fn test_finds_in_redis_and_renews_ttl() {
        let key = TestKey::new();
        let val = TestStruct::new();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        svc.clone().set::<_, _>(&key, &val).await.expect("fine");
        let found: TestStruct = try_redis_fallback_db(
            svc.clone(),
            key,
            || async {return Err(DataError::Unexpected("test".into()));},
            true
        ).await.expect("should give back struct");
        assert_eq!(found.field, val.field);
    }

    #[test]
    async fn test_finds_in_db() {
        let key = TestKey::new();
        let val = TestStruct::new();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        let found: TestStruct = try_redis_fallback_db(
            svc.clone(),
            key,
            || async {return Ok(val.clone())},
            true
        ).await.expect("should give back struct");
        assert_eq!(found.field, val.field);
    }

    #[test]
    async fn test_expire_timed_duration() {
        let key = TestKey::new();
        let val = TestStruct::new();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        svc.clone().set::<_,_>(&key, &val).await.expect("ok");
        let res: TestStruct = svc.clone().get::<_, _>(&key).await.expect("ok");
        assert_eq!(res, val);
        expire_key_timed_with_duration(
            svc.clone(),
            &key,
            1
        ).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let error: RedisError = svc.clone().get::<_, TestStruct>(&key).await.expect_err("key expired");
        assert_eq!(RedisError::NotFound("test".into()), error);
    }

    #[test]
    async fn test_expire_immediate() {
        let key = TestKey::new();
        let val = TestStruct::new();
        let svc: Arc<RedisService> = Arc::new(RedisService::new());
        svc.clone().set::<_, _>(&key, &val).await.expect("ok");
        let res: TestStruct = svc.clone().get::<_, _>(&key).await.expect("ok");
        assert_eq!(res, val);
        expire_immediate(
            svc.clone(),
            &key,
        ).await;
        let error: RedisError = svc.clone().get::<_, TestStruct>(&key).await.expect_err("key expired");
        assert_eq!(RedisError::NotFound("test".into()), error);
    }
}