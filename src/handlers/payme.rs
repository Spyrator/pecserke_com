use std::str::FromStr;

use payment_strings::{Currency, Payment, PaymentEncoding};
use rocket::{get, response::content::RawHtml};
use rocket::{post, FromForm};

use rocket::form::Form;

#[derive(FromForm)]
pub struct PaymentForm<'r> {
    iban: &'r str,
    recipient: &'r str,
    amount: u32,
    currency: &'r str,
    message: &'r str,
}

#[get("/<amount>/<message>")]
pub fn pay_me_amount_message(amount: u32, message: &str) -> RawHtml<String> {
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

    return RawHtml(svg_qr.into());
}

//
#[get("/<amount>")]
pub fn pay_me_amount(amount: u32) -> RawHtml<String> {
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

    return RawHtml(svg_qr.into());
}

// FORM
#[post("/form", data = "<payment>")]
pub fn pay_me_form(payment: Form<PaymentForm>) -> RawHtml<String> {
    let dpayment = Payment {
        iban: payment.iban.into(),
        swift: "".to_owned(),
        amount: payment.amount,
        currency: Currency::from_str(payment.currency).unwrap_or(Currency::CZK),
        message: payment.message.to_owned(),
        recipient_name: payment.recipient.to_owned(),
    };
    let payment_string = dpayment.string(&PaymentEncoding::SPD);
    let svg_qr = qr_from_str::svg(&payment_string);

    return RawHtml(svg_qr.into());
}
