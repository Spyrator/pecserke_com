use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

pub async fn get_index<'a>() -> Html<String> {
    return Html(HelloTemplate { name: "" }.to_string());
}
