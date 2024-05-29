use crate::entities:: questions::Model as Question;
use crate::*;
use askama::Template;
use askama_axum::IntoResponse;
use axum::debug_handler;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use std::sync::Arc;
use crate::models::question_model::get;
use tokio::sync::RwLock;

#[derive(Template, Serialize, Debug)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    /// A template struct for rendering an HTML page using the `index.html` template.
    ///
    /// # Fields
    ///
    /// * `question: Question`: A `Question` instance.
    ///
    /// # Template
    ///
    /// The template is located at `index.html`.
    question: Question,
}

/// Creates a new instance of `IndexTemplate` with the given `question`.
///
/// # Parameters
///
/// * `question: &'a Question`: A reference to a `Question` instance.
///
/// # Returns
///
/// A new `IndexTemplate` instance populated with the given `question`.
impl IndexTemplate {
    fn new(question: Question) -> Self {
        Self { question }
    }
}

/// Handles the index route
///
/// This function is responsible for handling the index route. It retrieves a
/// random question from the question bank and returns it in the response.
///
/// # Parameters
///
/// * `questions`: The question bank
///
/// # Returns
///
/// * A response containing the random question or a 404 error if no question is available
#[debug_handler]
pub async fn handler_index(State(questions): State<Arc<RwLock<QuestionBank>>>) -> Response {
    let read_lock = questions.read().await;
    let question = get(&read_lock.question_db, 1).await.unwrap();
    (StatusCode::OK, IndexTemplate::new(question)).into_response()
}
