use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct PaginationResponse {
    pub next_cursor: Option<String>,
}