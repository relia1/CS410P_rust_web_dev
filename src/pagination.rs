// use crate::*;
// use std::collections::HashMap;
// use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub limit: usize,
}
