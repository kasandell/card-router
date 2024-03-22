use std::future::Future;
use crate::error::api_error::ApiError;

pub async fn wrap_api_call_2<T, Fn, Fut>(f: Fn) -> Result<T, ApiError>
    where Fn: FnOnce() -> Fut,
    Fut: Future<Output=Result<T, ApiError>>
{
    f().await
}

pub fn wrap_api_call<T, Err>(result: Result<T, Err>) -> Result<T, ApiError>
    where ApiError: From<Err>
{
    result.map_err(|e| ApiError::from(e))
}
/*
pub async fn wrap_api_call_3<T>(result: Future<Output=Result<T, ApiError>>) -> Future<Output=Result<T, ApiError>>
{
    result
}

 */