use crate::*;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    /// A template struct for rendering an HTML page using the `index.html` template.
    ///
    /// # Fields
    ///
    /// * `question: &'a Question`: A reference to a `Question` instance.
    ///
    /// # Template
    ///
    /// The template is located at `index.html`.
    question: &'a Question,
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
impl<'a> IndexTemplate<'a> {
    fn new(question: &'a Question) -> Self {
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
    match questions.read().await.get_random() {
        Some(question) => (StatusCode::OK, IndexTemplate::new(question)).into_response(),
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
