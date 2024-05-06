#![warn(clippy::all)]

mod api;
mod db_config;
mod pagination;
mod question;
mod questionbank;
mod web;

//use ::migration::*;
use api::*;
use db_config::*;
use pagination::*;
use question::*;
use questionbank::*;
use web::*;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres, Row};
use std::error::Error;
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
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

async fn run() -> Result<(), Box<dyn Error>> {
    /*
    use std::env::var;
    let pg_user = var("PG_USER");
    let password = "password".trim();
    let pg_host = var("PG_HOST");
    let pg_dbname = var("PG_DBNAME");
    let url = format!(
        "postgres://{:?}:{:?}@{:?}:5432/{:?}",
        pg_user, password, pg_host, pg_dbname,
    );
    let db = Database::connect(url.clone()).await?;
    let _db = &db
        .execute(Statement::from_string(
            db.get_database_backend(),
            format!("CREATE DATABASE {:?};", var("PG_DBNAME"),),
        ))
        .await?;

    tracing::trace!("test\ntest");
    let url = format!("{:?}/{:?}", url.clone(), var("PG_DBNAME"));
    let connection = Database::connect(&url).await;

    Ok(())
    */
    Ok(())
    //todo!("run fn");
}

async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

#[tokio::main]
async fn main() {
    let db = Arc::new(RwLock::new(
        db_setup().await.expect("Unable to setup the database"),
    ));
    dbg!(db);
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

    // Load questions into the server
    let questionsbank = QuestionBank::new().await.unwrap();
    /*QuestionBank::new("assets/questions.json").unwrap_or_else(|e| {
        tracing::error!("question new: {}", e);
        std::process::exit(1);
    });
    */
    let questionsbank = Arc::new(RwLock::new(questionsbank));
    // todo!("loading questions");

    // routes with their handlers
    let apis = Router::new()
        .route("/questions", get(questions))
        .route("/paginated_questions", get(paginated_questions))
        .route("/questions/:id", get(get_question))
        .route("/question", get(question))
        .route("/questions/add", post(post_question))
        .route("/questions/:id", delete(delete_question))
        .route("/questions/:id", put(update_question));

    // handy openai auto generated docs!
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
    let redoc_ui = Redoc::with_url("/redoc", ApiDoc::openapi());
    let rapidoc_ui = RapiDoc::new("/api-docs/openapi.json").path("/rapidoc");

    let app = Router::new()
        .route("/", get(handler_index))
        .route("/index.html", get(handler_index))
        .merge(swagger_ui)
        .merge(redoc_ui)
        .merge(rapidoc_ui)
        .nest("/api/v1", apis)
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
            //.route_service("/favicon.ico", favicon)
        );

    let question_db = db_setup().await;
    // start up webserver on localhost:3000
    let ip = SocketAddr::new([0, 0, 0, 0].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    todo!("axum::serve");
}
