use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct Pagination {
    pub page: u64,
    pub limit: u64,
}
