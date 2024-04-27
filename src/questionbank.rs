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
type QuestionMap = HashMap<String, Question>;

/// A question bank that stores and manages questions
/// The question bank is saved/loaded from a file on disk
#[derive(Debug)]
pub struct QuestionBank {
    file: File,
    questionmap: QuestionMap,
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
    pub fn new<P: AsRef<std::path::Path>>(db_path: P) -> Result<Self, std::io::Error> {
        let mut file = File::create_new(&db_path)
            .and_then(|mut f| {
                let questionmap: QuestionMap = HashMap::new();
                let json = serde_json::to_string(&questionmap).unwrap();
                f.write_all(json.as_bytes())?;
                f.sync_all()?;
                f.rewind()?;
                Ok(f)
            })
            .or_else(|e| {
                if e.kind() == ErrorKind::AlreadyExists {
                    File::options().read(true).write(true).open(&db_path)
                } else {
                    Err(e)
                }
            })?;
        let json = std::io::read_to_string(&mut file)?;
        let questionmap = serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
        Ok(Self { file, questionmap })
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
    pub fn paginated_get(
        &self,
        page: usize,
        limit: usize,
    ) -> Result<Vec<(String, &Question)>, QuestionBankErr> {
        let total_questions = self.questionmap.len();
        let start_index = (page - 1) * limit;
        let end_index = start_index + limit;
        if start_index > total_questions {
            return Err(QuestionBankErr::QuestionPaginationInvalid(
                "Invalid query parameter values".to_string(),
            ));
        }
        let question_vec: Vec<(_, _)> = self
            .questionmap
            .iter()
            .enumerate()
            .filter(|&(i, _)| i >= start_index && i < end_index)
            .map(|(_, (k, v))| (k.clone(), v))
            .collect();
        dbg!(&question_vec);
        Ok(question_vec)
    }

    /// Retrieves a random question.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to a random `Question` or `None` if the question bank is empty.
    pub fn get_random(&self) -> Option<&Question> {
        fastrand::choice(self.questionmap.iter()).map(|x| x.1)
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
    pub fn get<'a>(&'a self, index: &str) -> Result<&'a Question, QuestionBankErr> {
        self.questionmap
            .get(index)
            .ok_or_else(|| QuestionBankErr::QuestionDoesNotExist(index.to_string()))
    }

    /// Writes the current state of the question bank to disk.
    ///
    /// This method serializes the `questionmap` to JSON and writes it to the file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the write operation was successful or had
    /// an IO error
    fn write_questions(&mut self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.questionmap).unwrap();
        self.file.rewind()?;
        self.file.set_len(0)?;
        self.file.write_all(json.as_bytes())?;
        self.file.sync_all()
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
        let id = question.id.clone();
        if self.questionmap.get(&id).is_some() {
            return Err(QuestionBankErr::QuestionExists(id));
        }
        self.questionmap.insert(id, question);
        self.write_questions()?;
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
    pub fn delete(&mut self, index: &str) -> Result<(), QuestionBankErr> {
        if !self.questionmap.contains_key(index) {
            return Err(QuestionBankErr::QuestionDoesNotExist(index.to_string()));
        }
        self.questionmap.remove(index);
        self.write_questions()?;
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
    pub fn update(
        &mut self,
        index: &str,
        question: Question,
    ) -> Result<StatusCode, QuestionBankErr> {
        if !self.questionmap.contains_key(index) {
            return Err(QuestionBankErr::NoQuestionPayload);
        }
        if question.id.is_empty() {
            return Err(QuestionBankErr::QuestionUnprocessable(index.to_string()));
        }
        self.questionmap
            .entry(index.to_string())
            .and_modify(|x| *x = question);
        self.write_questions()?;
        Ok(StatusCode::OK)
    }
}

/// Converts a `QuestionBank` reference into an HTTP response.
///
/// # Returns
///
/// A `Response` object with a status code of 200 OK and a JSON body containing the question map.
impl IntoResponse for &QuestionBank {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.questionmap)).into_response()
    }
}
