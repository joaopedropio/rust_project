use std::fmt;
use std::fmt::Formatter;
use uuid::Uuid;
use std::str::FromStr;
use fmt::Display;

pub enum PaymentBrand {
    None,
    MasterCard,
    Visa,
    Amex,
}

impl Display for PaymentBrand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PaymentBrand::None => write!(f, "none"),
            PaymentBrand::MasterCard => write!(f, "mastercard"),
            PaymentBrand::Visa => write!(f, "visa"),
            PaymentBrand::Amex => write!(f, "amex"),
        }
    }
}

impl FromStr for PaymentBrand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amex" => Ok(PaymentBrand::Amex),
            "mastercard" => Ok(PaymentBrand::MasterCard),
            "visa" => Ok(PaymentBrand::Visa),
            "none" => Ok(PaymentBrand::None),
            _ => Err(())
        }
    }
}

pub enum PaymentMethod {
    Credit,
    Debit,
    Boleto,
    Money,
}

impl Display for PaymentMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PaymentMethod::Credit => write!(f, "credit"),
            PaymentMethod::Debit => write!(f, "debit"),
            PaymentMethod::Boleto => write!(f, "boleto"),
            PaymentMethod::Money => write!(f, "money"),
        }
    }
}

impl FromStr for PaymentMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "credit" => Ok(PaymentMethod::Credit),
            "debit" => Ok(PaymentMethod::Credit),
            "boleto" => Ok(PaymentMethod::Credit),
            "money" => Ok(PaymentMethod::Credit),
            _ => Err(()),
        }
    }
}

pub struct Payment {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub amount: i64,
    pub installment: i32,
    pub method: PaymentMethod,
    pub brand: PaymentBrand,
}

impl Payment {
    pub fn new() -> Payment {
        Payment {
            id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            seller_id: Uuid::new_v4(),
            amount: 0,
            installment: 0,
            method: PaymentMethod::Credit,
            brand: PaymentBrand::None,
        }
    }
}
