// Deriving `Debug` and `Clone` traits for the `Config` struct
// - `Debug` allows the struct to be printed using the debug format (`{:?}`)
// - `Clone` enables creating a duplicate of a `Config` instance
#[derive(Debug, Clone)]
pub struct Config{
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16
}

// Implementing functions for `Config`
impl Config{

    pub fn init() -> Config{
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        Config{
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port: 3100
        }
    }
}

// ! unwrap is called on the Result returned by parse.
// ! If the Result is Ok(i64), unwrap will extract and return the integer value inside.
// ! If the Result is Err, unwrap will cause the program to panic, immediately stopping execution and printing an error message with details about the failure.
// ! In this case, unwrap is used to simplify handling by panicking if jwt_maxage can’t be parsed into an integer.