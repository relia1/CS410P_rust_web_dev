use crate::models::lib::*;
use axum::response::IntoResponse;
use serde::{ser::SerializeStruct, Serialize, Serializer};

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
