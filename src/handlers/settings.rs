use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "settings.html")]
pub struct SettingsTemplate {}

#[get("/settings")]
pub fn get_settings() -> SettingsTemplate {
    return SettingsTemplate {};
}
