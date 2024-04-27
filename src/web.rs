use crate::*;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    question: &'a Question,
}

impl<'a> IndexTemplate<'a> {
    fn new(question: &'a Question) -> Self {
        Self { question }
    }
}

pub async fn handler_index(State(questions): State<Arc<RwLock<QuestionBank>>>) -> Response {
    match questions.read().await.get_random() {
        Some(question) => (StatusCode::OK, IndexTemplate::new(question)).into_response(),
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
