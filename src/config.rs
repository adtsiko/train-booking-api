pub struct Config {
    pub port: u16,
    pub data_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("APP_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .expect("APP_PORT must be a number");

        let data_path = std::env::var("DATA_PATH").unwrap_or_else(|_| "train.json".to_string());

        Self { port, data_path }
    }
}
