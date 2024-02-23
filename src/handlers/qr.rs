use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "qr.html")]
pub struct QrTemplate {}

pub async fn get_qr() -> Html<String> {
    return Html(QrTemplate {}.to_string());
}
