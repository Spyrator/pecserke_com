use std::str::FromStr;

use axum::{extract::Path, Form};
use payment_strings::{Currency, Payment, PaymentEncoding};

#[derive(serde::Deserialize)]
pub struct PaymentForm {
    iban: String,
    recipient: String,
    amount: u32,
    currency: String,
    message: String,
}

pub async fn pay_me_amount_message(Path(amount): Path<u32>, Path(message): Path<String>) -> String {
    let payment = Payment {
        iban: "CZ8120100000002000466782".into(),
        swift: "FIOBCZPPXXX".to_owned(),
        amount,
        currency: Currency::CZK,
        message: message.to_owned(),
        recipient_name: "Filip Pecsérke".to_owned(),
    };
    let payment_string = payment.string(&PaymentEncoding::SPD);
    let svg_qr = qr_from_str::svg(&payment_string);

    return svg_qr.into();
}

pub async fn pay_me_amount(Path(amount): Path<u32>) -> String {
    let payment = Payment {
        iban: "CZ8120100000002000466782".into(),
        swift: "FIOBCZPPXXX".to_owned(),
        amount,
        currency: Currency::CZK,
        message: "".to_owned(),
        recipient_name: "Filip Pecsérke".to_owned(),
    };
    let payment_string = payment.string(&PaymentEncoding::SPD);
    let svg_qr = qr_from_str::svg(&payment_string);

    return svg_qr.into();
}

// FORM
// #[post("/form", data = "<payment>")]
pub async fn pay_me_form(Form(payment): Form<PaymentForm>) -> String {
    let dpayment = Payment {
        iban: payment.iban.into(),
        swift: "".to_owned(),
        amount: payment.amount,
        currency: Currency::from_str(&payment.currency).unwrap_or(Currency::CZK),
        message: payment.message.to_owned(),
        recipient_name: payment.recipient.to_owned(),
    };
    let payment_string = dpayment.string(&PaymentEncoding::SPD);
    let svg_qr = qr_from_str::svg(&payment_string);

    return svg_qr.into();
}
