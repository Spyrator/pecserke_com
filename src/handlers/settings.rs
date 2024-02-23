use askama::Template;
use axum::response::Html;

use super::template::WebsiteTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
pub struct SettingsTemplate {}

pub async fn get_settings() -> Html<String> {
    let settings = SettingsTemplate {}.to_string();
    return Html(SettingsTemplate {}.to_string());
}
