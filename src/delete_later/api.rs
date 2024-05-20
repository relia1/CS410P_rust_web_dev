/*
use crate::question::Question;
use crate::*;

// From utoipa/examples/{simple-axum, axum-todo}.

#[derive(OpenApi)]
#[openapi(
    paths(
        questions,
        question,
        get_question,
        post_question,
        delete_question,
        update_question,
    ),
    components(
        schemas(Question, QuestionBankError)
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

    let all_questions = questions.read();
    match all_questions.await.paginated_get(page, limit).await {
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
    path = "/api/v1/question",
    responses(
        (status = 200, description = "Return random question", body = Question),
        (status = 204, description = "Questionbase is empty")
    )
)]
pub async fn question(State(questions): State<Arc<RwLock<QuestionBank>>>) -> Response {
    match questions.read().await.get_random() {
        Some(question) => question.into_response(),
        None => QuestionBankError::response(
            StatusCode::NO_CONTENT,
            Box::new(QuestionBankErr::DoesNotExist("".to_string())),
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
    match questions.read().await.get(question_id).await {
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
    match questions.write().await.add(question).await {
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
    match questions.write().await.delete(question_id).await {
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
    match questions.write().await.update(question_id, question).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => QuestionBankError::response(StatusCode::BAD_REQUEST, e),
    }
}
*/
