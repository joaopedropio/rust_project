extern crate dotenv;
pub use self::environment::Environment;

pub mod environment {
    pub struct Environment {
        pub postgres_password: String,
    }

    impl Environment {
        pub fn build() -> Environment {
            dotenv::dotenv().ok();
            Environment {
                postgres_password: std::env::var("DATABASE_URL").unwrap(),
            }
        }
    }
}
