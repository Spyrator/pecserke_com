// #![allow(unused)]

mod handlers;

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

use shuttle_axum::ShuttleAxum;

#[shuttle_runtime::main]
async fn init() -> ShuttleAxum {
    let router = Router::new()
        .route("/", get(get_index))
        .route("/settings", get(get_settings))
        .route("/qr", get(get_qr))
        .route("/payme/:amount", get(pay_me_amount))
        .route("/payme/:amount/:message", get(pay_me_amount_message))
        .route("/payme/form", post(pay_me_form));
    Ok(router.into())
}
