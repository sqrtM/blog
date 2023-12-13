use crate::views::blog_view::{BlogPostView, BlogSummaryView};
use askama::Template;

use crate::views::board_view::BoardView;
use crate::views::reply_view::ReplyView;
use crate::views::thread_view::ThreadView;

pub mod blog_view;
pub mod board_view;
pub mod reply_view;
pub mod thread_view;

mod filters {
    use askama::Error;
    use chrono::{DateTime, Utc};

    pub fn replace_reply_syntax(input: &str) -> Result<String, Error> {
        let regex = regex::Regex::new(r#"&gt;&gt;([a-zA-Z0-9-]+)"#).unwrap();
        Ok(regex
            .replace_all(input, "<a href='#reply-$1'>&raquo;$1</a>")
            .to_string())
    }

    pub fn format_utc_datetime(dt: &DateTime<Utc>) -> Result<String, Error> {
        Ok(dt.format("%Y-%m-%d %H:%M (%Z)").to_string())
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "components/random-fact.html")]
pub struct RandomFactTemplate {
    pub(crate) question: String,
    pub(crate) answer: String,
}

#[derive(Template)]
#[template(path = "forum/boards.html")]
pub struct AllBoardsPage {
    pub boards: Vec<BoardView>,
}

#[derive(Template)]
#[template(path = "forum/threads.html")]
pub struct ThreadsPage {
    pub(crate) board: BoardView,
    pub(crate) threads: Vec<ThreadView>,
}

#[derive(Template)]
#[template(path = "forum/components/thread.html")]
pub struct NewThread {
    pub(crate) thread: ThreadView,
}

#[derive(Template)]
#[template(path = "forum/components/reply.html")]
pub struct NewReply {
    pub(crate) reply: ReplyView,
}

#[derive(Template)]
#[template(path = "blog/blog-post.html")]
pub struct BlogPostPage {
    pub(crate) post: BlogPostView,
}

#[derive(Template)]
#[template(path = "blog/blog-posts.html")]
pub struct AllBlogPostsPage {
    pub(crate) posts: Vec<BlogSummaryView>,
}
