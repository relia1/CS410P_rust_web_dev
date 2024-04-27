use crate::*;

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
    fn from(e: std::io::Error) -> Self {
        QuestionBankErr::QuestionBankIoError(e.to_string())
    }
}

#[derive(Debug)]
pub struct QuestionBankError {
    pub status: StatusCode,
    pub error: QuestionBankErr,
}

impl<'s> ToSchema<'s> for QuestionBankError {
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

impl Serialize for QuestionBankError {
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
    pub fn response(status: StatusCode, error: QuestionBankErr) -> Response {
        let error = QuestionBankError { status, error };
        (status, Json(error)).into_response()
    }
}

type QuestionMap = HashMap<String, Question>;

#[derive(Debug)]
pub struct QuestionBank {
    file: File,
    questionmap: QuestionMap,
}

impl QuestionBank {
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

    pub fn get_random(&self) -> Option<&Question> {
        fastrand::choice(self.questionmap.iter()).map(|x| x.1)
    }

    pub fn get<'a>(&'a self, index: &str) -> Result<&'a Question, QuestionBankErr> {
        self.questionmap
            .get(index)
            .ok_or_else(|| QuestionBankErr::QuestionDoesNotExist(index.to_string()))
    }

    fn write_questions(&mut self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.questionmap).unwrap();
        self.file.rewind()?;
        self.file.set_len(0)?;
        self.file.write_all(json.as_bytes())?;
        self.file.sync_all()
    }

    pub fn add(&mut self, question: Question) -> Result<(), QuestionBankErr> {
        let id = question.id.clone();
        if self.questionmap.get(&id).is_some() {
            return Err(QuestionBankErr::QuestionExists(id));
        }
        self.questionmap.insert(id, question);
        self.write_questions()?;
        Ok(())
    }

    pub fn delete(&mut self, index: &str) -> Result<(), QuestionBankErr> {
        if !self.questionmap.contains_key(index) {
            return Err(QuestionBankErr::QuestionDoesNotExist(index.to_string()));
        }
        self.questionmap.remove(index);
        self.write_questions()?;
        Ok(())
    }

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

impl IntoResponse for &QuestionBank {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.questionmap)).into_response()
    }
}
