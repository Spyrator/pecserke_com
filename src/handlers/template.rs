use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct WebsiteTemplate {
    // pub main: &'a str,
}
