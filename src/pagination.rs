// use crate::*;
// use std::collections::HashMap;
// use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub limit: usize,
}

// ex `/questions?start=1&end=10`
/*
pub async fn extract_pagination(
    State(questions): State<Arc<RwLock<QuestionBank>>>,
    Query(params): Query<Pagination>,
) -> Response {
    let questions = questions.read().await;
    questions.get("id1");

    let page = params.page;
    let limit = params.limit;
    let start = (page - 1) * limit;
    let end = start + limit;
    end.into_response()
}
*/
