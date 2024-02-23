use axum::response::Html;

use super::template::WebsiteTemplate;

pub async fn get_index<'a>() -> Html<String> {
    return Html(WebsiteTemplate {}.to_string());
}
