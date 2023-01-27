pub use self::dependencies::Dependencies;

mod dependencies {
    use crate::repository::PaymentRepo;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Error, Pool, Postgres};

    #[derive(Clone)]
    pub struct Dependencies {
        pub payment_repo: PaymentRepo,
    }

    impl Dependencies {
        fn create_database_pool(connection_string: &str) -> Result<Pool<Postgres>, Error> {
            PgPoolOptions::new()
                .max_connections(10)
                .connect_lazy(connection_string)
        }

        pub fn new(connection_string: &str) -> Dependencies {
            let pool = match Dependencies::create_database_pool(connection_string) {
                Ok(pool) => pool,
                Err(err) => panic!("can't connect to database: {:?}", err),
            };
            Dependencies {
                payment_repo: PaymentRepo::new(pool),
            }
        }
    }
}
