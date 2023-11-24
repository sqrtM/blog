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
    //testing
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => {}
    }

    let app_state = AppState {
        db: get_pool().await,
    };

    let router = Router::new()
        .nest("/", routes())
        .nest("/api", api_routes())
        .with_state(app_state);

    axum::Server::bind(&std::env::var("BIND_ADDR").expect("BIND_ADDR must be set").parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum_test::TestServer;

    use crate::AppState;
    use crate::repositories::get_pool;
    use crate::routes::{api_routes, routes};

    #[tokio::test]
    async fn it_should_get() {
        dotenvy::dotenv().expect("mfw no env");

        let app_state = AppState {
            db: get_pool().await,
        };

        let app = Router::new()
            .nest("/", routes())
            .nest("/api", api_routes())
            .with_state(app_state);


        let server = TestServer::new(app).unwrap();

        let response = server
            .get("/users")
            .await;

        assert_eq!(response.text(), "root user");
    }
}
