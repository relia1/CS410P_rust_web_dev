#![warn(clippy::all)]

mod api;
mod pagination;
mod question;
mod questionbank;
mod web;

use api::*;
use pagination::*;
use question::*;
use questionbank::*;
use web::*;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::fs::File;
use std::io::{ErrorKind, Seek, Write};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};
use tracing_subscriber::{fmt, EnvFilter};
extern crate serde_json;
extern crate thiserror;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};

use utoipa::{
    openapi::schema::{ObjectBuilder, Schema, SchemaType},
    openapi::RefOr,
    OpenApi, ToSchema,
};

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{self, sync::RwLock};
extern crate fastrand;
extern crate tracing;
use askama::Template;
use tracing::{error, info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[tokio::main]
async fn main() {
    let fmt_layer = fmt::layer().with_file(true).with_line_number(true).pretty();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let questionsbank = QuestionBank::new("assets/questions.json").unwrap_or_else(|e| {
        tracing::error!("question new: {}", e);
        std::process::exit(1);
    });
    let questionsbank = Arc::new(RwLock::new(questionsbank));

    let apis = Router::new()
        .route("/questions", get(questions))
        .route("/paginated_questions", get(paginated_questions))
        .route("/questions/:id", get(get_question))
        .route("/question", get(question))
        .route("/questions/add", post(post_question))
        .route("/questions/:id", delete(delete_question))
        .route("/questions/:id", put(update_question));

    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
    let redoc_ui = Redoc::with_url("/redoc", ApiDoc::openapi());
    let rapidoc_ui = RapiDoc::new("/api-docs/openapi.json").path("/rapidoc");

    info!("info");
    trace!("trace");
    let app = Router::new()
        .route("/", get(handler_index))
        .route("/index.html", get(handler_index))
        //.route_service("/favicon.ico", favicon)
        .merge(swagger_ui)
        .merge(redoc_ui)
        .merge(rapidoc_ui)
        .nest("/api/v1", apis)
        /*
        .layer(trace_layer)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
                .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT]),
        )
        */
        .with_state(questionsbank)
        .fallback(handler_404)
        .layer(
            ServiceBuilder::new().layer(trace_layer).layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
                    .expose_headers(Any),
            ),
        );

    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
