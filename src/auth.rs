// use libsql::{params, Connection};
// use rocket::{form::Form, get, http::CookieJar, post, response::content::RawHtml, FromForm, State};

// #[derive(FromForm)]
// pub struct LoginForm<'r> {
//     user: &'r str,
//     pass: &'r str,
// }

// #[post("/login", data = "<login>")]
// pub async fn login(
//     login: Form<LoginForm<'_>>,
//     conn: &State<Connection>,
//     cookies: &CookieJar<'_>,
// ) -> RawHtml<String> {
//     let x = conn
//         .query(
//             "SELECT CAST(id as text) FROM users where name=?1 and password=?2;",
//             // "select cast(id as text) from users where name='filip' and password='highbred';"
//             params![login.user, login.pass],
//         )
//         .await;

//     let x = match x {
//         Ok(mut a) => a.next().unwrap_or_default(),
//         Err(_) => {
//             println!("cant get the next row");
//             None
//         }
//     };

//     if let Some(a) = x {
//         let user_id: String = a
//             .get::<String>(0)
//             .unwrap_or("Couldn't get the first column".into());
//         cookies.add_private(("user_id", user_id.clone()));
//         return RawHtml(format!("hi {}", &user_id));
//     }
//     return RawHtml(format!("hi!"));
// }

// #[get("/user")]
// pub async fn user(cookies: &CookieJar<'_>) -> RawHtml<String> {
//     let id = cookies.get_private("user_id");
//     match id {
//         Some(user_id) => {
//             return RawHtml(format!("User id: {}", user_id.value()));
//         }
//         None => {
//             return RawHtml("User not logged in ".to_string());
//         }
//     };
// }

// #[get("/users")]
// pub async fn users(conn: &State<Connection>) -> RawHtml<String> {
//     let x = conn.query("SELECT * FROM users;", ()).await;

//     let mut x = match x {
//         Ok(x) => x,
//         Err(e) => {
//             println!("{:?}", e);
//             return RawHtml(format!("Fail! Couldn't get rows:/ \n{:?}", e));
//         }
//     };

//     loop {
//         let next = match x.next() {
//             Ok(a) => a,
//             Err(e) => {
//                 println!("{}", e);
//                 return RawHtml(format!("Fail! {}", e));
//             }
//         };

//         match next {
//             Some(row) => println!("{:?}", row),
//             None => break,
//         }
//     }

//     return RawHtml(format!("hi!"));
// }
