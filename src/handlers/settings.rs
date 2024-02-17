use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "settings.html")]
pub struct SettingsTemplate {}

pub async fn get_settings() -> Html<String> {
    return Html(SettingsTemplate {}.to_string());
}
