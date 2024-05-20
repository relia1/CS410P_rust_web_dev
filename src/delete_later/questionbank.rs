/*
use crate::question::Question;
use crate::*;

/// An enumeration of errors that may occur
#[derive(Debug, thiserror::Error, ToSchema, Serialize)]
pub enum QuestionBankErr {
    #[error("Questionbank io failed: {0}")]
    IoError(String),
    #[error("Question {0} doesn't exist")]
    DoesNotExist(String),
    #[error("Invalid query parameter values")]
    PaginationInvalid(String),
}

impl From<std::io::Error> for QuestionBankErr {
    /// Converts a `std::io::Error` into a `QuestionBankErr`.
    ///
    /// # Description
    ///
    /// This allows `std::io::Error` instances to be converted into
    /// `QuestionBankErr`, wrapping the I/O error as a `QuestionBankIoError`.
    ///
    /// # Example
    ///
    /// ```
    /// let io_err = std::io::Error::new(std::io::ErrorKind::Other, "IO error");
    /// let question_bank_err: QuestionBankErr = io_err.into();
    /// ```
    fn from(e: std::io::Error) -> Self {
        QuestionBankErr::IoError(e.to_string())
    }
}

/// struct that represents a QuestionBank error, but include a `StatusCode`
/// in addition to a `QuestionBankErr`
#[derive(Debug)]
pub struct QuestionBankError {
    pub status: StatusCode,
    pub error: String,
}

/// Implements `ToSchema` trait for `QuestionBankError` generating a JSON schema
/// for the error type
impl<'s> ToSchema<'s> for QuestionBankError {
    /// Returns a JSON schema for `QuestionBankError`
    ///
    /// The schema defines two properties:
    ///
    /// * `status`: A string representing the HTTP status code associated with the error.
    /// * `error`: A string describing the specific error that occurred.
    fn schema() -> (&'s str, RefOr<Schema>) {
        let sch = ObjectBuilder::new()
            .property(
                "status",
                ObjectBuilder::new().schema_type(SchemaType::String),
            )
            .property(
                "error",
                ObjectBuilder::new().schema_type(SchemaType::String),
            )
            .example(Some(serde_json::json!({
                "status":"404","error":"no question"
            })))
            .into();
        ("QuestionBankError", sch)
    }
}

/// Implements the `Serialize` trait for `QuestionBankError`
impl Serialize for QuestionBankError {
    /// Serializes a `QuestionBankError`
    ///
    /// The serialized JSON object will have two properties:
    ///
    /// * `status`: A string for the HTTP status code
    /// * `error`: A string describing the error
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let status: String = self.status.to_string();
        let mut state = serializer.serialize_struct("QuestionBankError", 2)?;
        state.serialize_field("status", &status)?;
        state.serialize_field("error", &self.error)?;
        state.end()
    }
}

impl QuestionBankError {
    /// Creates a `Response` instance from a `StatusCode` and `QuestionBankErr`.
    ///
    /// # Parameters
    ///
    /// * `status`: The HTTP status code.
    /// * `error`: The `QuestionBankErr` instance.
    ///
    /// # Returns
    ///
    /// `Response` instance with the status code and JSON body containing the error.
    pub fn response(status: StatusCode, error: Box<dyn Error>) -> Response {
        let error = QuestionBankError {
            status,
            error: error.to_string(),
        };
        (status, Json(error)).into_response()
    }
}

/// A type alias for a `Pool<Postgres>`
type QuestionDB = Pool<Postgres>;

/// A question bank that stores and manages questions
/// The question bank is saved/loaded from a file on disk
#[derive(Debug)]
pub struct QuestionBank {
    question_db: QuestionDB,
}

impl QuestionBank {
    /// Creates a new `QuestionBank` instance.
    ///
    /// # Parameters
    ///
    /// * `db_path`: The path to the file that will store the questions.
    ///
    /// # Returns
    ///
    /// A new `QuestionBank` instance, or an error if the database cannot be initialized
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let question_db = db_setup().await?;

        Ok(Self { question_db })
    }

    /// Retrieves a paginated list of questions from the question bank.
    ///
    /// # Parameters
    ///
    /// * `page`: The page number to retrieve (starts at 1)
    /// * `limit`: The number of questions to retrieve per page.
    ///
    /// # Returns
    ///
    /// A vector of Question's
    /// If the pagination parameters are invalid, returns a `QuestionBankErr` error.
    pub async fn paginated_get(
        &self,
        page: i32,
        limit: i32,
    ) -> Result<Vec<question::Question>, Box<dyn Error>> {
        let row = sqlx::query(r#"SELECT COUNT(*) FROM questions;"#)
            .fetch_one(&self.question_db)
            .await?;
        let total_questions: i64 = row.get(0);
        let start_index = (page - 1) * limit;
        if (start_index as i64) > total_questions {
            return Err(Box::new(QuestionBankErr::PaginationInvalid(
                "Invalid query parameter values".to_string(),
            )));
        }
        let questions = sqlx::query(
            r#"
            SELECT q.id, q.title, q.content, ARRAY_AGG(t.name) AS tags
            FROM questions q
            LEFT JOIN question_tags qt ON q.id = qt.question_id
            LEFT JOIN tags t ON qt.tag_id = t.id
            GROUP BY q.id, q.title, q.content
            ORDER BY q.id
            LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(start_index)
        .fetch_all(&self.question_db)
        .await?;

        let mut question_vec: Vec<Question> = Vec::new();
        for row in questions {
            question_vec.push(<Question as std::convert::From<PgRow>>::from(row));
        }

        Ok(question_vec)
    }

    /// Retrieves a random question.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to a random `Question` or `None` if the question bank is empty.
    pub fn get_random(&self) -> Option<&Question> {
        /*
        fastrand::choice(self.question_db.iter()).map(|x| x.1)
        */
        todo!("get of random question");
    }

    /// Retrieves a question by its ID.
    ///
    /// # Parameters
    ///
    /// * `index`: The ID of the question.
    ///
    /// # Returns
    ///
    /// A reference to the `Question` instance with the specified ID, or a `QuestionBankErr` error if the question does not exist.
    pub async fn get(&self, index: i32) -> Result<Question, Box<dyn Error>> {
        let question = sqlx::query(
            r#"
            SELECT q.id, q.title, q.content, ARRAY_AGG(t.name) AS tags
            FROM questions q
            LEFT JOIN question_tags qt ON q.id = qt.question_id
            LEFT JOIN tags t ON qt.tag_id = t.id
            WHERE q.id = $1
            GROUP BY q.id, q.title, q.content;
            "#,
        )
        .bind(index)
        .fetch_one(&self.question_db)
        .await?;

        Ok(<Question as std::convert::From<PgRow>>::from(question))
    }

    /// Adds a new question.
    ///
    /// # Parameters
    ///
    /// * `question`: The `Question` to add to the question bank.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the question was added successfully.
    /// If the question already exists, returns a `QuestionBankErr` error.
    pub async fn add(&mut self, question: Question) -> Result<(), Box<dyn Error>> {
        let question_to_insert =
            sqlx::query(r#"INSERT INTO questions (title, content) VALUES ($1, $2) RETURNING id"#)
                .bind(question.title)
                .bind(question.content)
                .fetch_one(&self.question_db)
                .await?;

        let question_id: i32 = question_to_insert.get(0);

        let mut tag_id_vec: Vec<i32> = Vec::new();
        if question.tags.is_some() {
            let mut tag_id;
            for tag in &question.tags.clone().unwrap() {
                tag_id = sqlx::query(r#"INSERT INTO tags (name) VALUES ($1) RETURNING id"#)
                    .bind(tag)
                    .fetch_one(&self.question_db)
                    .await?;

                tag_id_vec.push(tag_id.get(0));
            }
        }

        for tag_id in tag_id_vec {
            sqlx::query(r#"INSERT INTO question_tags (question_id, tag_id) VALUES ($1, $2);"#)
                .bind(question_id)
                .bind(tag_id)
                .execute(&self.question_db)
                .await?;
        }

        Ok(())
    }

    /// Removes a question by its ID.
    ///
    /// # Parameters
    ///
    /// * `index`: The ID of the question.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the question was removed successfully.
    /// If the question does not exist, returns a `QuestionBankErr` error.
    pub async fn delete(&mut self, index: i32) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            DELETE FROM questions
            WHERE id IN (
              SELECT question_id FROM question_tags
              WHERE question_id = $1
            );
            "#,
        )
        .bind(index)
        .execute(&self.question_db)
        .await?;

        Ok(())
    }

    /// Updates a question by its ID.
    ///
    /// # Parameters
    ///
    /// * `index`: The ID of the question to update.
    /// * `question`: The updated `Question` instance.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the question was updated successfully.
    /// If the question does not exist or is unprocessable, returns a `QuestionBankErr` error.
    /// If successful, returns a `StatusCode` of 200.
*   pub async fn update(
        &mut self,
        index: i32,
        question: Question,
    ) -> Result<Question, Box<dyn Error>> {
        let title = question.title;
        let content = question.content;
        let tags = question.tags;

        let mut question_to_update = self.get(index).await?;
        question_to_update.title = title.clone();
        question_to_update.content = content.clone();
        question_to_update.tags = tags;

        sqlx::query(
            r#"
            UPDATE questions
            SET title = $1, content = $2
            WHERE id = $3;"#,
        )
        .bind(title)
        .bind(content)
        .bind(index)
        .execute(&self.question_db)
        .await?;

        sqlx::query(
            r#"
            DELETE FROM question_tags
            WHERE question_id = $1;
            "#,
        )
        .bind(question_to_update.id)
        .execute(&self.question_db)
        .await?;

        let mut tag_id_vec: Vec<i32> = Vec::new();
        let mut tag_id;
        if let Some(ref tags_to_add) = question_to_update.tags {
            for tag in tags_to_add {
                tag_id = sqlx::query(r#"INSERT INTO tags (name) VALUES ($1) RETURNING id"#)
                    .bind(tag)
                    .fetch_one(&self.question_db)
                    .await?;

                tag_id_vec.push(tag_id.get(0));
            }
            for tag_id in tag_id_vec {
                sqlx::query(r#"INSERT INTO question_tags (question_id, tag_id) VALUES ($1, $2);"#)
                    .bind(index)
                    .bind(tag_id)
                    .execute(&self.question_db)
                    .await?;
            }
        };

        Ok(question_to_update)
    }
}
*/
