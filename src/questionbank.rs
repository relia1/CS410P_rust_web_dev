use crate::*;

/// An enumeration of errors that may occur
#[derive(Debug, thiserror::Error, ToSchema, Serialize)]
pub enum QuestionBankErr {
    #[error("Question already exists: {0}")]
    QuestionExists(String),
    #[error("Questionbank io failed: {0}")]
    QuestionBankIoError(String),
    #[error("Missing question payload")]
    NoQuestionPayload,
    #[error("Question {0} doesn't exist")]
    QuestionDoesNotExist(String),
    #[error("Question payload unprocessable")]
    QuestionUnprocessable(String),
    #[error("Invalid query parameter values")]
    QuestionPaginationInvalid(String),
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
        QuestionBankErr::QuestionBankIoError(e.to_string())
    }
}

/// struct that represents a QuestionBank error, but include a `StatusCode`
/// in addition to a `QuestionBankErr`
#[derive(Debug)]
pub struct QuestionBankError {
    pub status: StatusCode,
    pub error: QuestionBankErr,
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
    pub fn response(status: StatusCode, error: QuestionBankErr) -> Response {
        let error = QuestionBankError { status, error };
        (status, Json(error)).into_response()
    }
}

/// A type alias for a `HashMap` of question IDs mapped to `Question` instances.
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
    /// A new `QuestionBank` instance, or an error if the file cannot be created or read.
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
    /// A vector of tuples that are the question ID and reference to the question
    /// If the pagination parameters are invalid, returns a `QuestionBankErr` error.
    pub async fn paginated_get(
        &self,
        page: usize,
        limit: usize,
    ) -> Result<Vec<(String, &Question)>, Box<dyn Error>> {
        /*
        let total_questions = self.question_db. //.len();
        let start_index = (page - 1) * limit;
        let end_index = start_index + limit;
        if start_index > total_questions {
            return Err(QuestionBankErr::QuestionPaginationInvalid(
                "Invalid query parameter values".to_string(),
            ));
        }
        let question_vec: Vec<(_, _)> = self
            .question_db
            .iter()
            .enumerate()
            .filter(|&(i, _)| i >= start_index && i < end_index)
            .map(|(_, (k, v))| (k.clone(), v))
            .collect();
        dbg!(&question_vec);
        Ok(question_vec)
        */
        let row = sqlx::query(r#"SELECT COUNT(*) FROM questions;"#)
            .fetch_one(&self.question_db)
            .await?;
        let total_questions: i64 = row.get(0);
        let start_index = (page - 1) * limit;
        if start_index as i64 > total_questions {
            return Err(Box::new(QuestionBankErr::QuestionPaginationInvalid(
                "Invalid query parameter values".to_string(),
            )));
        }
        let questions = sqlx::query(
            r#"
            SELECT q.title, q.content, ARRAY_AGG(t.name) AS tags
            FROM questions q
            JOIN question_tags qt ON q.id = qt.question_id
            JOIN tags t ON qt.tag_id = t.id
            GROUP BY q.id, q.title, q.content
            ORDER BY q.id
            OFFSET $1
            FETCH NEXT $2 ROWS ONLY
            "#,
        )
        .bind(limit as i32)
        .bind(start_index as i32)
        .fetch_all(&self.question_db)
        .await?;

        for row in questions {
            let title: String = row.get(0);
            tracing::info!(title);
        }

        todo!("pagination");
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
    pub async fn get<'a>(&'a self, index: &str) -> Result<&'a Question, QuestionBankErr> {
        /*
        self.question_db
            .get(index)
            .ok_or_else(|| QuestionBankErr::QuestionDoesNotExist(index.to_string()))
        */
        let question = sqlx::query(
            r#"
            SELECT q.title, q.content, ARRAY_AGG(t.name) AS tags
            FROM questions q
            JOIN question_tags qt ON q.id = qt.question_id
            JOIN tags t ON qt.tag_id = t.id
            WHERE q.id = $1
            GROUP BY q.id, q.title, q.content;
            "#,
        )
        .bind(index)
        .fetch_one(&self.question_db)
        .await;
        todo!("get of question");
    }

    /// Writes the current state of the question bank to disk.
    ///
    /// This method serializes the `question_db` to JSON and writes it to the file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the write operation was successful or had
    /// an IO error
    fn write_questions(&mut self) -> Result<(), std::io::Error> {
        /*
        let json = serde_json::to_string(&self.question_db).unwrap();
        self.file.rewind()?;
        self.file.set_len(0)?;
        self.file.write_all(json.as_bytes())?;
        self.file.sync_all()
        */
        todo!("writing of question to db");
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
    pub fn add(&mut self, question: Question) -> Result<(), QuestionBankErr> {
        /*
        let id = question.id.clone();
        if self.question_db.get(&id).is_some() {
            return Err(QuestionBankErr::QuestionExists(id));
        }
        self.question_db.insert(id, question);
        self.write_questions()?;
        Ok(())
        */
        todo!("Add of question");
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
    pub fn delete(&mut self, index: &str) -> Result<(), QuestionBankErr> {
        /*
        if !self.question_db.contains_key(index) {
            return Err(QuestionBankErr::QuestionDoesNotExist(index.to_string()));
        }
        self.question_db.remove(index);
        self.write_questions()?;
        Ok(())
        */
        todo!("delete of question");
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
    pub fn update(
        &mut self,
        index: &str,
        question: Question,
    ) -> Result<StatusCode, QuestionBankErr> {
        /*
        if !self.question_db.contains_key(index) {
            return Err(QuestionBankErr::NoQuestionPayload);
        }
        if question.id.is_empty() {
            return Err(QuestionBankErr::QuestionUnprocessable(index.to_string()));
        }
        self.question_db
            .entry(index.to_string())
            .and_modify(|x| *x = question);
        self.write_questions()?;
        Ok(StatusCode::OK)
        */
        todo!("update of question");
    }
}

/*
/// Converts a `QuestionBank` reference into an HTTP response.
///
/// # Returns
///
/// A `Response` object with a status code of 200 OK and a JSON body containing the question map.
*/
/*
impl IntoResponse for &QuestionBank {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.question_db)).into_response()
    }
}
todo!("check into response trait");
*/
