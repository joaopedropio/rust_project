extern crate core;

mod domain;
mod service;
mod api;
mod environment;
mod dependencies;
mod repository;

use environment::Environment;
use dependencies::Dependencies;
use api::routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let env = Environment::build();
    let dep = Dependencies::new(&env.postgres_password);
    let mut server = tide::with_state(State::new(dep));
    routes::configure_routes(&mut server);
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub deps: Dependencies
}

impl State {
    pub fn new(dep: Dependencies) -> State {
        State { deps: dep }
    }
}
