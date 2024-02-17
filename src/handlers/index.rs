use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

#[get("/")]
pub fn get_index<'a>() -> HelloTemplate<'a> {
    return HelloTemplate { name: "" };
}
