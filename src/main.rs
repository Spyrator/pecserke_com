#![allow(unused)] 

mod handlers;

mod auth;
use auth::{login, user, users};
use handlers::index::get_index;
use libsql::Connection;
use handlers::payme::{pay_me_amount, pay_me_amount_message, pay_me_form};
use handlers::qr::get_qr;
use rocket::routes;
use handlers::settings::get_settings;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn init(
    #[ shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_turso::Turso(
        addr = "{secrets.TURSO_ADDR}",
        local_addr = "{secrets.TURSO_ADDR}",
        token = "{secrets.TURSO_TOKEN}"
    )]
    client: Connection,
) -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .manage(client)
        .mount("/", routes![get_index, get_settings, get_qr])
        .mount("/auth/", routes![login, user, users])
        .mount(
            "/payme/",
            routes![pay_me_amount_message, pay_me_amount, pay_me_form],
        );

    Ok(rocket.into())
}
