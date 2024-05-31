#![ allow(warnings)]
use crate::controllers::lib::*;
use sea_orm::TryIntoModel;
use crate::entities::prelude::Answer;
use crate::models::answer_model::*;
use crate::models::errors::QuestionBankError;
use crate::QuestionBank;


// From utoipa/examples/{simple-axum, axum-todo}.
#[derive(OpenApi)]
#[openapi(
    paths(
        get_answer,
        post_answer,
        delete_answer,
        update_answer,
    ),
    components(
        schemas(Answer, QuestionBankError)
    ),
    tags(
        (name = "Question's Answers API", description = "Question's Answer API")
    )
)]
// TODO figure out a better way of combining answers/questions openapi
pub struct ApiDoc2;

#[utoipa::path(
    get,
    path = "/api/v1/questions/{id}/answer",
    responses(
        (status = 200, description = "Return specified answer", body = Answer),
        (status = 404, description = "No answer with this question", body = QuestionBankError),
    )
)]
pub async fn get_answer(
    State(answers): State<Arc<RwLock<QuestionBank>>>,
    Path(question_id): Path<i32>,
) -> Response {
    let read_guard = answers.read().await;
    match get(&read_guard.question_db, question_id).await {
        Ok(answer) => answer.into_response(),
        _ => (StatusCode::NO_CONTENT, "No answer for that question").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/questions/{id}/answer",
    request_body(
        content = inline(Answer),
        description = "Answer to add"
    ),
    responses(
        (status = 201, description = "Added answer", body = ()),
        (status = 400, description = "Bad request", body = QuestionBankError)
    )
)]
pub async fn post_answer(
    State(answers): State<Arc<RwLock<QuestionBank>>>,
    Json(answer): Json<Answer>,
) -> Response {
    let write_guard = answers.write().await;
    tracing::info!("post answer");
    match add(&write_guard.question_db, answer).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::BAD_REQUEST, e),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/questions/{id}/answer",
    responses(
        (status = 200, description = "Deleted answer", body = ()),
        (status = 400, description = "Bad request", body = QuestionBankError),
    )
)]
pub async fn delete_answer(
    State(answers): State<Arc<RwLock<QuestionBank>>>,
    Path(question_id): Path<i32>,
) -> Response {
    tracing::info!("delete answer");
    let write_guard = answers.write().await;
    match delete(&write_guard.question_db, question_id).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::BAD_REQUEST, e),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/questions/{id}/answer",
    request_body(
        content = inline(Answer),
        description = "Question to update"
    ),
    responses(
        (status = 200, description = "Updated answer", body = ()),
        (status = 400, description = "Bad request", body = QuestionBankError),
        (status = 404, description = "Answer not found", body = QuestionBankError),
        (status = 422, description = "Unprocessable entity", body = QuestionBankError),
    )
)]
#[debug_handler]
pub async fn update_answer(
    State(answers): State<Arc<RwLock<QuestionBank>>>,
    Path(question_id): Path<i32>,
    Json(answer): Json<Answer>,
) -> Response {
    let write_guard = answers.write().await;
    match update(&write_guard.question_db, question_id, answer).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::BAD_REQUEST, e),
    }
}
