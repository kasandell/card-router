use std::future::Future;
use crate::error::api_error::ApiError;

pub fn wrap_api_call<T, Err>(result: Result<T, Err>) -> Result<T, ApiError>
    where ApiError: From<Err>
{
    result.map_err(|e| ApiError::from(e))
}


#[cfg(test)]
mod test {
    use crate::error::api_error::ApiError;
    use crate::test_helper::error::serde_error;
    use crate::util::api_call::wrap_api_call;

    #[test]
    fn test_wrapper_error() {
        let err: Result<i32, serde_json::Error> = Err(serde_error());
        assert_eq!(
            ApiError::Unexpected("test".into()),
            wrap_api_call(err).expect_err("should convert to error")
        );
    }

    #[test]
    fn test_wrapper_ok() {
        let ok: Result<i32, serde_json::Error> = Ok(1);
        assert_eq!(
            1,
            wrap_api_call(ok).expect("should be ok")
        );
    }
}