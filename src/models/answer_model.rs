use sea_orm::{DbBackend, DeleteResult, FromQueryResult, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveModelTrait;
use crate::entities::answers::Entity;
use sea_orm::EntityTrait;
use crate::models::lib::*;
use crate::entities::answers::{Model as Answer, ActiveModel};

/// Retrieves an answer by its ID.
///
/// # Parameters
///
/// * `index`: The ID of the answer.
///
/// # Returns
///
/// An instance of an answer with the specified question ID, or a `QuestionBankErr` error if the answer does not exist.
pub async fn get(answers: &sea_orm::DatabaseConnection, index: i32) -> Result<Answer, Box<dyn Error>> {
    let answer = Answer::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT id, answer, question_id
        FROM answers
        WHERE question_id = $1
        "#,
        [index.into()],
        ))
        .one(answers)
        .await.unwrap_or(None);

    match answer {
        Some(res) => Ok(res),
        None => Err("No answer found for question".into()),
    }
}

/// Adds a new answer.
///
/// # Parameters
///
/// * `answer`: The `Answer` to add to the question bank.
///
/// # Returns
///
/// A `Result` indicating whether the answer was added successfully.
/// If the answer already exists, returns a `QuestionBankErr` error.
/// TODO maybe overwrite the answer if it exists?
pub async fn add(answers: &sea_orm::DatabaseConnection, answer: Answer) -> Result<(), Box<dyn Error>> {
    let answer_to_insert = ActiveModel
    {
        answer: Set(answer.answer),
        question_id: Set(answer.question_id),
        ..Default::default()
    };

    let question_id = Entity::insert(answer_to_insert).exec(answers).await?;
    tracing::debug!("ID of the question the answer was added to: {}", question_id.last_insert_id);

    Ok(())
}

/// Removes an answer by associated with a question
///
/// # Parameters
///
/// * `index`: The ID of the question.
///
/// # Returns
///
/// A `Result` indicating whether the answer was removed successfully.
/// If the answer does not exist, returns a `QuestionBankErr` error.
/// TODO need to look into what is expected here
pub async fn delete(answers: &sea_orm::DatabaseConnection, index: i32) -> Result<(), Box<dyn Error>> {
    let number_rows_deleted: DeleteResult = Entity::delete_by_id(index).exec(answers).await?;
    tracing::debug!("Deleted {:?} rows", number_rows_deleted);

    Ok(())
}

/// Updates an answer associated with a question.
///
/// # Parameters
///
/// * `index`: The ID of the question to update.
/// * `answer`: The updated `Answer` instance.
///
/// # Returns
///
/// A `Result` indicating whether the question's answer was updated successfully.
/// If the question does not exist or is unprocessable, returns a `QuestionBankErr` error.
/// If successful, returns a `StatusCode` of 200.
pub async fn update(
    answers: &sea_orm::DatabaseConnection,
    index: i32,
    answer: Answer,
) -> Result<Answer, Box<dyn Error>> {
    let db_answer = Entity::find_by_id(index).one(answers).await?;
    if db_answer.is_none() {
        Err("No answer found for question".into())
    } else {
        let mut db_answer: ActiveModel = db_answer.unwrap().into();
        db_answer.answer = Set(answer.answer);
        Ok(db_answer.update(answers).await?)
    }
}
