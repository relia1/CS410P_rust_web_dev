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
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let listener = tokio::net::TcpListener::bind(&"127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
