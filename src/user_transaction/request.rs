use serde::Deserialize;

#[derive(Deserialize)]
pub struct CursorQueryParams {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}