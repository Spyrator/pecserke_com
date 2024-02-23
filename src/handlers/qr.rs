use askama::Template;
use axum::response::Html;

use super::template::WebsiteTemplate;

#[derive(Template)]
#[template(path = "qr.html")]
pub struct QrTemplate {}

pub async fn get_qr() -> Html<String> {
    let qr = QrTemplate {}.to_string();
    return Html(QrTemplate {}.to_string());
}
