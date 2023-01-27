use crate::domain::Payment;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct PaymentRepo {
    pool: Pool<Postgres>,
}

impl PaymentRepo {
    pub fn new(pool: Pool<Postgres>) -> PaymentRepo {
        PaymentRepo { pool }
    }

    pub async fn create(&self, pmt: Payment) {
        let query = format!(
            "\
INSERT INTO payment.payments (id, buyer_id, seller_id, amount, installment, method, brand, created_at) \
values ('{}', '{}', '{}', {}, {}, '{}', '{}', '{}');\
        ",
            pmt.id, pmt.buyer_id, pmt.seller_id, pmt.amount, pmt.installment, pmt.method, pmt.brand, pmt.created_at.to_string());
        match sqlx::query(&query).execute(&self.pool).await {
            Ok(_) => println!("deu bom no banco"),
            Err(err) => println!("deu ruim no banco {}", err),
        }
    }
}
