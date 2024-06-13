use crate::{
    entities::{question, question::Question},
    models::lib::*,
};

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
    questions: &Pool<Postgres>,
    page: i32,
    limit: i32,
) -> Result<Vec<question::Question>, Box<dyn Error>> {
    let row = sqlx::query(r#"SELECT COUNT(*) FROM questions;"#)
        .fetch_one(questions)
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
    .fetch_all(questions)
    .await?;

    let mut question_vec: Vec<Question> = Vec::new();
    for row in questions {
        question_vec.push(<Question as std::convert::From<PgRow>>::from(row));
    }

    Ok(question_vec)
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
pub async fn get(questions: &Pool<Postgres>, index: i32) -> Result<Vec<Question>, Box<dyn Error>> {
    let mut question_vec = vec![];
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
    .fetch_one(questions)
    .await?;

    question_vec.push(<Question as std::convert::From<PgRow>>::from(question));
    Ok(question_vec)
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
pub async fn add(questions: &Pool<Postgres>, question: Question) -> Result<(), Box<dyn Error>> {
    let question_to_insert =
        sqlx::query(r#"INSERT INTO questions (title, content) VALUES ($1, $2) RETURNING id"#)
            .bind(question.title)
            .bind(question.content)
            .fetch_one(questions)
            .await?;

    let question_id: i32 = question_to_insert.get(0);

    let mut tag_id_vec: Vec<i32> = Vec::new();
    if question.tags.is_some() {
        let mut tag_id;
        for tag in &question.tags.clone().unwrap() {
            tag_id = sqlx::query(r#"INSERT INTO tags (name) VALUES ($1) RETURNING id"#)
                .bind(tag)
                .fetch_one(questions)
                .await?;

            tag_id_vec.push(tag_id.get(0));
        }
    }

    for tag_id in tag_id_vec {
        sqlx::query(r#"INSERT INTO question_tags (question_id, tag_id) VALUES ($1, $2);"#)
            .bind(question_id)
            .bind(tag_id)
            .execute(questions)
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
pub async fn delete(questions: &Pool<Postgres>, index: i32) -> Result<(), Box<dyn Error>> {
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
    .execute(questions)
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
    questions: &Pool<Postgres>,
    index: i32,
    question: Question,
) -> Result<Vec<Question>, Box<dyn Error>> {
    let title = question.title;
    let content = question.content;
    let tags = question.tags;

    let mut question_to_update = get(questions, index).await?;
    question_to_update[0].title.clone_from(&title);
    question_to_update[0].content.clone_from(&content);
    question_to_update[0].tags = tags;

    sqlx::query(
        r#"
        UPDATE questions
        SET title = $1, content = $2
        WHERE id = $3;"#,
    )
    .bind(title)
    .bind(content)
    .bind(index)
    .execute(questions)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM question_tags
        WHERE question_id = $1;
        "#,
    )
    .bind(question_to_update[0].id)
    .execute(questions)
    .await?;

    let mut tag_id_vec: Vec<i32> = Vec::new();
    let mut tag_id;
    if let Some(ref tags_to_add) = question_to_update[0].tags {
        for tag in tags_to_add {
            tag_id = sqlx::query(r#"INSERT INTO tags (name) VALUES ($1) RETURNING id"#)
                .bind(tag)
                .fetch_one(questions)
                .await?;

            tag_id_vec.push(tag_id.get(0));
        }
        for tag_id in tag_id_vec {
            sqlx::query(r#"INSERT INTO question_tags (question_id, tag_id) VALUES ($1, $2);"#)
                .bind(index)
                .bind(tag_id)
                .execute(questions)
                .await?;
        }
    };

    Ok(question_to_update)
}
