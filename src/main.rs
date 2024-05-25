#![warn(clippy::all)]

mod controllers;
mod entities;
mod config;
mod models;
mod pagination;
mod repositories;
mod web;
mod db_config;

use web::*;
use config::*;

use crate::controllers::question_controller::*;
use crate::controllers::answer_controller::*;
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::trace;
use tracing_subscriber::{fmt, EnvFilter};
extern crate serde_json;
extern crate thiserror;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};

use utoipa::OpenApi;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{self, sync::RwLock};
extern crate fastrand;
extern crate tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[tokio::main]
async fn main() {
    // Setup formatting and environment for trace
    let fmt_layer = fmt::layer().with_file(true).with_line_number(true).pretty();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
    // https://carlosmv.hashnode.dev/adding-logging-and-tracing-to-an-axum-app-rust

    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new())
        .on_response(trace::DefaultOnResponse::new());

    // Connect to database
    let questionsbank = Arc::new(RwLock::new(QuestionBank::new().await.unwrap()));
    // let questionsbank = Arc::new(RwLock::new(questionsbank));

    // routes with their handlers
    let apis = Router::new()
        .route("/questions", get(questions))
        .route("/questions/:id", get(get_question))
        .route("/questions/add", post(post_question))
        .route("/questions/:id", delete(delete_question))
        .route("/questions/:id", put(update_question));

    // handy openai auto generated docs!
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
    let redoc_ui = Redoc::with_url("/redoc", ApiDoc::openapi());
    let rapidoc_ui = RapiDoc::new("/api-docs/openapi.json").path("/rapidoc");
    let swagger_ui2 = SwaggerUi::new("/swagger-ui2").url("/api-docs/openapi2.json", ApiDoc2::openapi());
    let redoc_ui2 = Redoc::with_url("/redoc2", ApiDoc2::openapi());
    let rapidoc_ui2 = RapiDoc::new("/api-docs/openapi.json2").path("/rapidoc2");


    let app = Router::new()
        .route("/", get(handler_index))
        .route("/index.html", get(handler_index))
        .nest("/api/v1", apis)
        .merge(swagger_ui)
        .merge(redoc_ui)
        .merge(rapidoc_ui)
        .merge(swagger_ui2)
        .merge(redoc_ui2)
        .merge(rapidoc_ui2)

        .with_state(questionsbank)
        .fallback(handler_404)
        .layer(
            ServiceBuilder::new().layer(trace_layer), /*.layer(
                                                          CorsLayer::new()
                                                              .allow_origin(Any)
                                                              .allow_methods(Any)
                                                              .allow_headers(Any)
                                                              .expose_headers(Any),

                                                      ),*/ //.route_service("/favicon.ico", favicon)
        );

    // start up webserver on localhost:3000
    let ip = SocketAddr::new([0, 0, 0, 0].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
