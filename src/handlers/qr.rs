use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "qr.html")]
pub struct SettingsTemplate {}

#[get("/qr")]
pub fn get_qr() -> SettingsTemplate {
    return SettingsTemplate {};
}
