use sqlx::{Postgres, Pool};
use crate::domain::{Payment, PaymentBrand, PaymentMethod};
use uuid::Uuid;

pub struct PaymentRepo {
    pool: Pool<Postgres>
}

impl PaymentRepo {
    pub fn new(pool: Pool<Postgres>) -> PaymentRepo {
        PaymentRepo {
            pool,
        }
    }

    pub async fn create(&self, payment: Payment) {
        let query = format!("\
INSERT INTO payment.payments (id, buyer_id, seller_id, amount, installment, method, brand) \
values ({}, {}, {}, {}, {}, {}, {});\
        ", Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), 1234, 1, PaymentMethod::Credit.to_string(), PaymentBrand::MasterCard.to_string());
        match sqlx::query_as(&query).fetch_one(&self.pool).await {
            Ok(_) => println!("deu bom no banco"),
            Err(_) => println!("deu ruim no banco"),
        }
    }
}