use std::borrow::BorrowMut;
use crate::domain::{Payment, PaymentBrand, PaymentMethod};
use crate::State;
use std::str::FromStr;
use chrono::{Utc, NaiveDate};
use tide::prelude::*;
use tide::{Request, Response};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct PaymentModel {
    pub id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub amount: i64,
    pub installment: i32,
    pub method: String,
    pub brand: String,
    pub created_at: String,
}

pub async fn create_payment_handler(mut req: Request<State>) -> tide::Result {
    let payment = parse_request(req.borrow_mut()).await;
    req.state().deps.payment_repo.create(payment).await;
    Ok(Response::new(201))
}

async fn parse_request(req: &mut Request<State>) -> Payment {
    let model: PaymentModel = match req.body_json().await {
        Ok(model) => model,
        Err(err) => {
            println!("{}", err);
            PaymentModel {
                id: "".to_string(),
                buyer_id: "".to_string(),
                seller_id: "".to_string(),
                amount: 0,
                installment: 0,
                method: "".to_string(),
                brand: "".to_string(),
                created_at: Utc::now().date_naive().to_string(),
            }
        }
    };
    return parse_payment_from_request(model).await;
}

async fn parse_payment_from_request(model: PaymentModel) -> Payment {
    Payment {
        id: Uuid::from_str(&model.id).unwrap(),
        buyer_id: Uuid::from_str(&model.buyer_id).unwrap(),
        seller_id: Uuid::from_str(&model.seller_id).unwrap(),
        amount: model.amount,
        installment: model.installment,
        method: PaymentMethod::from_str(&model.method).unwrap(),
        brand: PaymentBrand::from_str(&model.brand).unwrap(),
        created_at: NaiveDate::from_str(&model.created_at).unwrap(),
    }
}
