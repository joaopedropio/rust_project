extern crate core;

mod domain;
mod service;
mod api;
mod environment;
mod dependencies;
mod repository;

use environment::Environment;
use dependencies::Dependencies;
use api::routes::Api;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let env = Environment::build();
    let dep = Dependencies::new(&env.postgres_password);
    let mut server = tide::new();
    let app = Api::new(dep);
    app.configure_routes(&mut server);
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
