mod controllers;

use axum::{
    Router,
    routing::get,
};
use crate::controllers::user_controller::hello;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(hello).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hello"
}

async fn get_foo() {}

async fn post_foo() {}

async fn foo_bar() {}