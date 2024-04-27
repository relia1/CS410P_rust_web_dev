use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Question {
    #[schema(example = "id string")]
    pub id: String,
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Content!")]
    pub content: String,
    #[schema(example = r#"["history", "math"]"#)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashSet<String>>,
}

impl Question {
    /// Creates a new `Question` instance.
    ///
    /// # Parameters
    ///
    /// * `id`: ID for the question.
    /// * `title`: The title of the question.
    /// * `content`: The content of the question.
    /// * `tags`: An optional list of tags
    ///
    /// # Returns
    ///
    /// A new `Question` instance with the provided parameters.
    pub fn new(id: &str, title: &str, content: &str, tags: &[&str]) -> Self {
        let id = id.into();
        let title = title.into();
        let content = content.into();
        let tags: Option<HashSet<String>> = if tags.is_empty() {
            None
        } else {
            Some(tags.iter().copied().map(String::from).collect())
        };
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl IntoResponse for &Question {
    /// Converts a `&Question` into an HTTP response.
    ///
    /// # Returns
    ///
    /// A `Response` object with a status code of 200 OK and a JSON body containing the question data.
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}
