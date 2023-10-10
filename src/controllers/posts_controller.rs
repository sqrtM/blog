use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::models::post::add_post_request::AddPostRequest;
use crate::models::AddResponse;
use crate::repositories::posts::insert::insert;
use crate::AppState;

pub async fn add_post(
    State(state): State<AppState>,
    Json(request): Json<AddPostRequest>,
) -> AddResponse {
    match request.is_valid() {
        Ok(_) => match insert(&state.db, request).await {
            Ok(_) => AddResponse {
                status: StatusCode::ACCEPTED,
                message: String::from("good"),
            },
            Err(e) => AddResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: e.to_string(),
            },
        },
        Err(e) => AddResponse {
            status: StatusCode::BAD_REQUEST,
            message: "over here".parse().unwrap(),
        },
    }
}
