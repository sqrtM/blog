use askama::Template;

#[derive(Template)]
#[template(path = "test.html")]
pub struct BaseTemplate;
