CREATE SCHEMA payment;
CREATE TABLE IF NOT EXISTS payment.payments (
    id uuid NOT NULL,
    buyer_id uuid NOT NULL,
    seller_id uuid NOT NULL,
    amount bigint,
    installment integer,
    method text NOT NULL,
    brand text NOT NULL
);