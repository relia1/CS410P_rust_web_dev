use sqlx::Pool;
use sqlx::Postgres;
use std::error::Error;
use crate::db_config::*;


/// A question bank that stores and manages questions and their answers
#[derive(Debug)]
pub struct QuestionBank {
    pub question_db: Pool<Postgres>,
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
}
