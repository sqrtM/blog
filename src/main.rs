use std::sync::Arc;

use axum::{
    Router,
    routing::get,
};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::controllers::user_controller;

mod controllers;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app_state = Arc::new(AppState {
        db: match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                println!("Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        }
    });
    sqlx::migrate!().run(&app_state.db).await.expect("Migration failed!");


    let user_routes = Router::new()
        .route("/",
               get(user_controller::root)
                   .post(user_controller::add_user)
        );

    let api_routes = Router::new()
        .route("/", get(root))
        .nest("/users", user_routes);

    let router = Router::new()
        .nest("/api", api_routes)
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "root"
}
