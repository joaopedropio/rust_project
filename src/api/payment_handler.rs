use std::future::Future;
use std::str::FromStr;
use tide::{Error, Request, Response};
use tide::prelude::*;
use uuid::Uuid;
use crate::domain::{Payment, PaymentMethod, PaymentBrand};
use crate::dependencies::Dependencies;

pub struct PaymentHandler {
}

#[derive(Debug, Deserialize)]
struct PaymentModel {
    pub id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub amount: i64,
    pub installment: i32,
    pub method: String,
    pub brand: String,
}


impl PaymentHandler {
    pub fn new(dep: Dependencies) -> PaymentHandler {
        PaymentHandler {
            dep,
        }
    }

    pub async fn create_payment_handler(&self, req: Request<()>) -> tide::Result {
        let payment = PaymentHandler::parse_payment_from_request(req).await?;
        self.dep.payment_repo.create(payment).await;
        Ok(Response::new(201))
    }

    pub fn vamove(&self) -> fn(Request<()>) -> dyn Future<Output = tide::Result> {
        async move |req: Request<()>| {
            self.payment_handler.create_payment_handler(req).await
        }
    }

    async fn parse_payment_from_request(mut req: Request<()>) -> Result<Payment, Error> {
        let payment_model :PaymentModel = req.body_json().await?;
        let payment = Payment {
            id: Uuid::from_str(&payment_model.id).unwrap(),
            buyer_id: Uuid::from_str(&payment_model.buyer_id).unwrap(),
            seller_id: Uuid::from_str(&payment_model.seller_id).unwrap(),
            amount: payment_model.amount,
            installment: payment_model.installment,
            method: PaymentMethod::from_str(&payment_model.method).unwrap(),
            brand: PaymentBrand::from_str(&payment_model.brand).unwrap(),
        };
        Ok(payment)
    }
}

