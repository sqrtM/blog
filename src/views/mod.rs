use crate::views::post_view::PostView;
use askama::Template;

pub mod post_view;
pub mod reply_view;

#[derive(Template)]
#[template(path = "test.html")]
pub struct BaseTemplate;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct AllPostsPage {
    pub(crate) posts: Vec<PostView>,
}
