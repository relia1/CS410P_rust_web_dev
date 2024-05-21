# Rust Web Example

Robert Elia 2024

# Description from the syllabus

Course - TOP: Rust Web Development.

The Rust Programming Language is experiencing rapid adoption in part due to its ability to provide
easy, fast and safe programming for systems that span low-level and high-level concerns. Rust is
increasingly being used to write Web services, including back-ends, front-ends, microservices and
clients. A number of high-quality Rust web server and client frameworks are available to support
this activity. In this course students will learn to work with Rust and the Web.

# Project Overview

Simple API using Rust and the Axum framework that provides an API for creating, reading, updating,
and deleting questions.

# API Endpoints

- GET /questions: Retrieves a list of all questions
- GET /paginated_questions: Retrieves a paginated list of questions
- GET /question: Retrieves a random question
- GET /questions/{id}: Retrieves a question by ID
- POST /questions/add: Creates a new question
- DELETE /questions/{id}: Deletes a question by ID
- PUT /questions/{id}: Updates a question by ID

# Documentation

API documentation can be found at /swagger-ui, /redoc, and /rapidoc

# Running the APP

`cargo run`
Server will then be available at http://localhost:3000 or http://127.0.0.1:3000

Additionally, the server can be run with different levels of tracing
`RUST_LOG=error cargo run`
`RUST_LOG=info cargo run`
`RUST_LOG=debug cargo run`
`RUST_LOG=trace cargo run`
