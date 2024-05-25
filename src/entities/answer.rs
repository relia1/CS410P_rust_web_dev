use crate::entities::lib::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Answer {
    #[schema(example = 5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[schema(example = "Answer")]
    pub answer: String,
    #[schema(example = "5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<i32>,
}

impl From<PgRow> for Answer {
    fn from(single_row: PgRow) -> Self {
        let id: Option<i32> = single_row.get("id");
        tracing::trace!(id);

        let answer: String = single_row.get("answer");
        tracing::trace!(answer);

        let question_id: Option<i32> = single_row.get("question_id");
        tracing::trace!(id);

        Self {
            id,
            answer,
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
    /// * `answer`: The answer of the answer.
    /// * `tags`: An optional list of tags
    ///
    /// # Returns
    ///
    /// A new `Answer` instance with the provided parameters.
    pub fn new(_id: Option<i32>, answer: &str, question_id: Option<i32>) -> Self {
        let answer = answer.into();

        Self {
            id: None,
            answer,
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
