#![allow(unused)]

mod handlers;

mod auth;

// use auth::{login, user, users};
use axum::{
    routing::{get, post},
    Router,
};
use handlers::{
    index::get_index,
    payme::{pay_me_amount, pay_me_amount_message, pay_me_form},
    qr::get_qr,
    settings::get_settings,
};
use libsql::Connection;
use shuttle_axum::ShuttleAxum;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn init(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_turso::Turso(
        addr = "{secrets.TURSO_LOCAL_ADDR}",
        local_addr = "{secrets.TURSO_LOCAL_ADDR}",
        token = "{secrets.TURSO_TOKEN}"
    )]
    client: Connection,
) -> ShuttleAxum {
    //let state = Arc::new(Mutex::new(client));

    // let rocket = rocket::build()
    //     .manage(client)
    //     .mount("/auth/", routes![login, user, users])

    let router = Router::new()
        .route("/", get(get_index))
        .route("/settings", get(get_settings))
        .route("/qr", get(get_qr))
        .route("/payme/:amount", get(pay_me_amount))
        .route("/payme/:amount/:message", get(pay_me_amount_message))
        .route("/payme/form", post(pay_me_form));

    Ok(router.into())
}
