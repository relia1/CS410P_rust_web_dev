use crate::entities::lib::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Answer {
    #[schema(example = 5)]
    pub id: Option<i32>,
    #[schema(example = "Content")]
    pub content: String,
    #[schema(example = r#"["history", "math"]"#)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<i32>,
}

impl From<PgRow> for Answer {
    fn from(single_row: PgRow) -> Self {
        let id: Option<i32> = single_row.get("id");
        tracing::trace!(id);

        let content: String = single_row.get("content");
        tracing::trace!(content);

        let question_id: Option<i32> = single_row.get("question_id");
        tracing::trace!(id);

        Self {
            id,
            content,
            question_id,
        }
    }
}

impl Answer {
    /// Creates a new `Answer` instance.
    ///
    /// # Parameters
    ///
    /// * `id`: ID for the answer.
    /// * `title`: The title of the question.
    /// * `content`: The content of the answer.
    /// * `tags`: An optional list of tags
    ///
    /// # Returns
    ///
    /// A new `Answer` instance with the provided parameters.
    pub fn new(_id: Option<i32>, content: &str, question_id: Option<i32>) -> Self {
        let content = content.into();

        Self {
            id: None,
            content,
            question_id,
        }
    }
}

impl IntoResponse for &Answer {
    /// Converts a `&Answer` into an HTTP response.
    ///
    /// # Returns
    ///
    /// A `Response` object with a status code of 200 OK and a JSON body containing the answer data.
    fn into_response(self) -> Response {
        tracing::info!("{:?}", &self);
        (StatusCode::OK, Json(&self)).into_response()
    }
}
