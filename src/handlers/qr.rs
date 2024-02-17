use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "qr.html")]
pub struct SettingsTemplate {}

pub async fn get_qr() -> Html<String> {
    return Html(SettingsTemplate {}.to_string());
}
