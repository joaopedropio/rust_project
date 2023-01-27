use crate::api::payment_handler::{create_payment_handler};
use crate::State;

pub fn configure_routes(app: &mut tide::Server<State>) {
    app.at("/payment").post(create_payment_handler);
}
