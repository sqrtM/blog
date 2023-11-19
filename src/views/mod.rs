use askama::Template;

use crate::views::reply_view::ReplyView;
use crate::views::thread_view::ThreadView;

pub mod reply_view;
pub mod thread_view;

#[derive(Template)]
#[template(path = "test.html")]
pub struct BaseTemplate;

#[derive(Template)]
#[template(path = "threads.html")]
pub struct AllThreadsPage {
    pub(crate) threads: Vec<ThreadView>,
}

#[derive(Template)]
#[template(path = "thread.html")]
pub struct NewThread {
    pub(crate) thread: ThreadView,
}

#[derive(Template)]
#[template(path = "reply.html")]
pub struct NewReply {
    pub(crate) reply: ReplyView,
}
