use axum::extract::{Path, State};
use uuid::Uuid;

use crate::models::blog::blog_entity::{BlogEntity, BlogSummaryEntity};
use crate::views::blog_view::{BlogPostView, BlogSummaryView};
use crate::views::{AllBlogPostsPage, BlogPostPage};
use crate::AppState;

pub async fn get_post(State(state): State<AppState>, Path(blog_id): Path<Uuid>) -> BlogPostPage {
    let ent = BlogEntity::get_post(&state.db, blog_id).await.unwrap();
    let post = BlogPostView::from(ent);
    BlogPostPage { post }
}

pub async fn get_all(State(state): State<AppState>) -> AllBlogPostsPage {
    let ent = BlogSummaryEntity::get_all_summaries(&state.db).await.unwrap();
    let posts = ent.into_iter().map(BlogSummaryView::from).collect();
    AllBlogPostsPage { posts }
}
