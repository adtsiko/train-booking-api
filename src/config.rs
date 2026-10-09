pub struct Config {
    pub port: u16,
    pub data_path: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("APP_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .expect("APP_PORT must be a number");

        let data_path = std::env::var("DATA_PATH").unwrap_or_else(|_| "train.json".to_string());
        let default_database_url = "postgres://train:train@localhost:5432/train_booking";
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| default_database_url.to_string());

        Self {
            port,
            data_path,
            database_url,
        }
    }
}
