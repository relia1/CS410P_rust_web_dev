use crate::controllers::lib::*;
use crate::entities::prelude::Question;
use crate::models::question_model::*;
use crate::models::errors::*;
use crate::pagination::Pagination;
// From utoipa/examples/{simple-axum, axum-todo}.

#[derive(OpenApi)]
#[openapi(
    paths(
        questions,
        get_question,
        post_question,
        delete_question,
        update_question,
    ),
    components(
        schemas(crate::entities::questions::Model, QuestionBankError)
    ),
    tags(
        (name = "Questions Server API", description = "Questions Server API")
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api/v1/questions?page=1&limit=5",
    responses(
        (status = 200, description = "List questions", body = [Question]),
        (status = 404, description = "No questions in that range")
    )
)]
pub async fn questions(
    State(questions): State<Arc<RwLock<QuestionBank>>>,
    Query(params): Query<Pagination>,
) -> Response {
    let page = params.page;
    let limit = params.limit;

    let read_lock = questions.read().await;
    match paginated_get(&read_lock.question_db, page, limit).await {
        Ok(res) => {
            tracing::info!("{:?}", &res);
            Json(res).into_response()
        }
        Err(e) => QuestionBankError::response(
            StatusCode::NOT_FOUND,
            Box::new(QuestionBankErr::DoesNotExist(e.to_string())),
        ),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/questions/{id}",
    responses(
        (status = 200, description = "Return specified question", body = Question),
        (status = 404, description = "No question with this id", body = QuestionBankError),
    )
)]
pub async fn get_question(
    State(questions): State<Arc<RwLock<QuestionBank>>>,
    Path(question_id): Path<i32>,
) -> Response {
    let read_lock = questions.read().await;
    match get(&read_lock.question_db, question_id).await {
        Ok(question) => question.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::NOT_FOUND, e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/questions/add",
    request_body(
        content = inline(Question),
        description = "Question to add"
    ),
    responses(
        (status = 201, description = "Added question", body = ()),
        (status = 400, description = "Bad request", body = QuestionBankError)
    )
)]
pub async fn post_question(
    State(questions): State<Arc<RwLock<QuestionBank>>>,
    Json(question): Json<Question>,
) -> Response {
    tracing::info!("post question!");
    let write_lock = questions.write().await;
    match add(&write_lock.question_db, question).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::BAD_REQUEST, e),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/questions/{id}",
    responses(
        (status = 200, description = "Deleted question", body = ()),
        (status = 400, description = "Bad request", body = QuestionBankError),
    )
)]
pub async fn delete_question(
    State(questions): State<Arc<RwLock<QuestionBank>>>,
    Path(question_id): Path<i32>,
) -> Response {
    tracing::info!("delete question");
    let write_lock = questions.write().await;
    match delete(&write_lock.question_db, question_id).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::BAD_REQUEST, e),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/questions/{id}",
    request_body(
        content = inline(Question),
        description = "Question to update"
    ),
    responses(
        (status = 200, description = "Updated question", body = ()),
        (status = 400, description = "Bad request", body = QuestionBankError),
        (status = 404, description = "Question not found", body = QuestionBankError),
        (status = 422, description = "Unprocessable entity", body = QuestionBankError),
    )
)]
#[debug_handler]
pub async fn update_question(
    State(questions): State<Arc<RwLock<QuestionBank>>>,
    Path(question_id): Path<i32>,
    Json(question): Json<Question>,
) -> Response {
    let write_lock = questions.write().await;
    match update(&write_lock.question_db, question_id, question).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::BAD_REQUEST, e),
    }
}
