#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub db_address: String,
    pub riot_api_key: String
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        Self {
            address: dotenv::var("BACKEND_ADDRESS").unwrap_or(String::from("localhost:8080")),
            db_address: dotenv::var("DATABASE_URL").unwrap_or(String::from(
                "DATABASE_URL=postgres://admin:admin@localhost:5666/enjoyer",
            )),
            riot_api_key: dotenv::var("RGAPI").expect("no riot api key given")
        }
    }
}
