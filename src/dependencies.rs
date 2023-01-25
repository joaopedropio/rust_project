pub use self::dependencies::Dependencies;

mod dependencies {
    use sqlx::{Postgres, Pool};
    use crate::repository::PaymentRepo;
    use sqlx::postgres::{PgPoolOptions};

    pub struct Dependencies {
        pub payment_repo: PaymentRepo,
    }

    impl Dependencies {
        fn create_database_pool(connection_string: &str) -> Pool<Postgres> {
            PgPoolOptions::new()
                .max_connections(10)
                .connect_lazy(connection_string)
                .ok().unwrap()
        }

        pub fn new(connection_string: &str) -> Dependencies {
            Dependencies {
                payment_repo: PaymentRepo::new(Dependencies::create_database_pool(connection_string))
            }
        }
    }
}