use crate::models::lib::*;
use crate::entities::questions::Model as Question;
use crate::entities::tags::Model as Tag;
use crate::entities::tags::Entity as TagEntity;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryOrder;
use crate::entities::questions::Entity;
use crate::entities::questions::Column;
use super::errors::QuestionBankErr;


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
    questions: &DatabaseConnection,
    page: u64,
    limit: u64,
) -> Result<Vec<Question>, Box<dyn Error>> {

    let paginator = Entity::find()
        .order_by_asc(Column::Id)
        .paginate(questions, limit);

    let paginated_result = paginator.fetch_page(page - 1).await;
    match paginated_result {
        Ok(res) => Ok(res),
        Err(e) => Err(Box::new(e)),
    }
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
pub async fn get(questions: &DatabaseConnection, index: i32) -> Result<Option<Question>, Box<dyn Error>> {
    let found_question = Entity::find_by_id(index).one(questions).await?;
    match found_question {
        Some(val) => Ok(Some(val)),
        None => Err(Box::new(QuestionBankErr::DoesNotExist("No question by that ID".to_string()))),
    }
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
pub async fn add(questions: &DatabaseConnection, question: Question/*, tag: Option<Tag>*/) -> Result<(), Box<dyn Error>> {
    let question_to_insert = ActiveModel
    {
        title: Set(question.title),
        content: Set(question.content),
        ..Default::default()
    };
    todo!("finish this");

    /*
    let question_id = Entity::insert(question_to_insert).exec(questions).await?;
    let mut tag_id = 0;
    match tag {
        Some(res) => {
            tag_id = TagEntity::insert(res).exec(questions).await?;
        },
        None => {
        },
    };
    tracing::debug!("ID of the question the answer was added to: {}", question_id.last_insert_id);

    Ok(())*/
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
pub async fn delete(questions: &DatabaseConnection, index: i32) -> Result<(), Box<dyn Error>> {
    let number_rows_deleted =  Entity::delete_by_id(index).exec(questions).await?;

    tracing::debug!("Deleted {:?} rows", number_rows_deleted);
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
    questions: &DatabaseConnection,
    index: i32,
    question: Question,
) -> Result<Question, Box<dyn Error>> {
    let db_question = Entity::find_by_id(index).one(questions).await?;
    if db_question.is_none() {
        Err("Question not found".into())
    } else {
        let mut db_question: ActiveModel = db_question.unwrap().into();
        db_question.title = Set(question.title);
        db_question.content = Set(question.content);
        Ok(db_question.update(questions).await?)
    }
}
