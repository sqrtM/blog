use axum::Router;
use sqlx::{Pool, Postgres};

use crate::repositories::get_pool;
use crate::routes::{api_routes, routes};

mod controllers;
mod models;
mod repositories;
mod routes;
mod services;
mod views;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        db: get_pool().await,
    };

    let router = Router::new()
        .nest("/", routes())
        .nest("/api", api_routes())
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
