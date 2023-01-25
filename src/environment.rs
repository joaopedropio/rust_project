pub use self::environment::Environment;

pub mod environment {
    pub struct Environment {
        pub postgres_password: String,
    }

    impl Environment {
        pub fn build() -> Environment {
            dotenv::dotenv().ok();

            for (key, value) in std::env::vars() {
                println!("{}: {}", key, value);
            }
            Environment {
                postgres_password: String::from("mysecretpassword"),
            }
        }
    }
}
