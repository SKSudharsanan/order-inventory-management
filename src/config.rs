pub struct Config {
    pub database_url: String,
    pub server_addr: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let server_addr = std::env::var("SERVER_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        let jwt_secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        let jwt_expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<i64>()
            .expect("JWT_EXPIRATION_HOURS must be a number");

        Self {
            database_url,
            server_addr,
            jwt_secret,
            jwt_expiration_hours,
        }
    }
}