use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Question {
    #[schema(example = 5)]
    pub id: i32,
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Content!")]
    pub content: String,
    #[schema(example = r#"["history", "math"]"#)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl From<PgRow> for Question {
    fn from(single_row: PgRow) -> Self {
        let id: i32 = single_row.get("id");
        tracing::trace!(id);

        let title: String = single_row.get("title");
        tracing::trace!(title);

        let content: String = single_row.get("content");
        tracing::trace!(content);

        let tags = match single_row.try_get::<Vec<String>, _>("tags") {
            Ok(val) => Some(val),
            Err(_) => None,
        };

        Self {
            id,
            title,
            content,
            tags,
        }
    }
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
    pub fn new(id: i32, title: &str, content: &str, tags: &[&str]) -> Self {
        let title = title.into();
        let content = content.into();
        let tags: Option<Vec<String>> = if tags.is_empty() {
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
        tracing::info!("{:?}", &self);
        (StatusCode::OK, Json(&self)).into_response()
    }
}
