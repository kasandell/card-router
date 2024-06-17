use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct PaginationRequest {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}