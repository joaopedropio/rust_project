use tide::Request;
use crate::api::payment_handler::PaymentHandler;
use crate::dependencies::Dependencies;

pub struct Api {

    pub payment_handler: PaymentHandler,
}

impl Api {
    pub fn new(dep: Dependencies) -> Api {
        Api {
            payment_handler: PaymentHandler::new(dep)
        }
    }

    pub fn configure_routes(&self, app: &mut tide::Server<()>) {
        app.at("/payment").post(&self.payment_handler.create_payment_handler);
    }
}
