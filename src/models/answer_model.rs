use crate::models::lib::*;
use crate::entities::answer::*;
/// Retrieves a question by its ID.
///
/// # Parameters
///
/// * `index`: The ID of the question.
///
/// # Returns
///
/// A reference to the `Question` instance with the specified ID, or a `QuestionBankErr` error if the question does not exist.
pub async fn get(answers: &Pool<Postgres>, index: i32) -> Result<Answer, Box<dyn Error>> {
    let answer = sqlx::query(
        r#"
        SELECT id, answer, question_id
        FROM answers
        WHERE question_id = $1
        "#,
    )
    .bind(index)
    .fetch_one(answers)
    .await?;

    Ok(<Answer as std::convert::From<PgRow>>::from(answer))
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
pub async fn add(answers: &Pool<Postgres>, answer: Answer) -> Result<(), Box<dyn Error>> {
    tracing::info!("in add fn");
    let answer_to_insert =
        sqlx::query(r#"INSERT INTO answers (answer, question_id) VALUES ($1, $2) RETURNING id"#)
        .bind(answer.answer)
        .bind(answer.question_id)
        .fetch_one(answers)
        .await?;


    let _question_id: i32 = answer_to_insert.get(0);
    tracing::info!(_question_id);

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
pub async fn delete(answers: &Pool<Postgres>, index: i32) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
        DELETE FROM answers
        WHERE question_id = $1
        ;"#,
    )
    .bind(index)
    .execute(answers)
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
pub async fn update(
    answers: &Pool<Postgres>,
    index: i32,
    answer: Answer,
) -> Result<Answer, Box<dyn Error>> {
    let answer = answer.answer;

    let mut answer_to_update = get(answers, index).await?;
    answer_to_update.answer.clone_from(&answer);

    sqlx::query(
        r#"
        UPDATE answers
        SET answer = $1
        WHERE question_id = $2;"#,
    )
    .bind(answer)
    .bind(index)
    .execute(answers)
    .await?;

    Ok(answer_to_update)
}
