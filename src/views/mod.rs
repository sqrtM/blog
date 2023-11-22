use askama::Template;

use crate::views::reply_view::ReplyView;
use crate::views::thread_view::ThreadView;

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
