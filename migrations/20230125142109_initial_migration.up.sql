CREATE SCHEMA payment;
CREATE TABLE IF NOT EXISTS payment.payments (
    id uuid NOT NULL PRIMARY KEY,
    buyer_id uuid NOT NULL,
    seller_id uuid NOT NULL,
    amount bigint,
    installment INTEGER,
    method text NOT NULL,
    brand text NOT NULL,
    created_at TIMESTAMP NOT NULL
);