use axum::extract::{Path, State};
use uuid::Uuid;

use crate::models::blog::blog_entity::BlogEntity;
use crate::views::blog_view::BlogPostView;
use crate::views::BlogPost;
use crate::AppState;

pub async fn get_post(State(state): State<AppState>, Path(blog_id): Path<Uuid>) -> BlogPost {
    let ent = BlogEntity::get_post(&state.db, blog_id).await.unwrap();
    let post = BlogPostView::from(ent);
    BlogPost { post }
}
