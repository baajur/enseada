use serde::Deserialize;

use crate::http::error::ApiError;

pub mod error;
pub mod extractor;
pub mod guard;
pub mod header;
pub mod middleware;
pub mod responses;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    limit: Option<usize>,
    cursor: Option<String>,
}

impl PaginationQuery {
    pub fn limit(&self) -> usize {
        self.limit.unwrap_or(20)
    }

    pub fn cursor(&self) -> Option<&String> {
        self.cursor.as_ref()
    }
}
