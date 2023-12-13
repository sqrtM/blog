use axum::routing::get;
use axum::Router;
use rand::prelude::SliceRandom;

use crate::routes::blog::blog_routes;
use crate::routes::board::board_routes;
use crate::routes::replies::replies_routes;
use crate::routes::threads::forum_routes;
use crate::routes::users::user_routes;
use crate::views::{HomeTemplate, RandomFactTemplate};
use crate::AppState;

mod blog;
mod board;
mod replies;
mod threads;
mod users;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/random-fact", get(random_fact))
        .nest("/boards", board_routes())
        .nest("/users", user_routes())
        .nest("/forum", forum_routes())
        .nest("/replies", replies_routes())
        .nest("/blog", blog_routes())
}
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
        .nest("/forum", forum_routes())
        .nest("/replies", replies_routes())
}

async fn root() -> HomeTemplate {
    HomeTemplate
}

async fn random_fact() -> RandomFactTemplate {
    let facts = vec![
        ("favorite dominant substitution", "bviΔ7"),
        ("favorite Doom 2 map", "E2M4"),
        ("favorite common chip flavor", "sour cream and onion"),
        ("favorite murakami novel", "wild sheep chase"),
        ("favorite bach fugue", "wtc ii: cm"),
        ("favorite coltrane", "ALS II: resolution"),
        ("favorite esoteric language", "orca"),
        ("favorite cpu", "ARM7TDMI"),
        ("favorite symphony", "mahler 3"),
        ("favorite non-prime integer", "0x5F3759DF"),
        ("favorite prime integer", "0x7FFFFFFF"),
        ("favorite jrpg", "dragon quest v"),
    ];
    let rand_fact = facts.choose(&mut rand::thread_rng()).unwrap();
    RandomFactTemplate {
        question: rand_fact.0.parse().unwrap(),
        answer: rand_fact.1.parse().unwrap(),
    }
}
