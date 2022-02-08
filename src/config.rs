use std::env;
use dotenv::dotenv;

#[allow(dead_code)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub listen_binding: String
}

#[allow(dead_code)]
impl Config {
    pub fn get() -> Result<Self, &'static str> {
        dotenv().ok();
        Ok(Self {
            db_host: env::var("POSTGRES_HOST").map_err(|_| "Config: POSTGRES_HOST not set")?,
            db_port: env::var("POSTGRES_PORT").map_err(|_| "Config: POSTGRES_PORT not set")?.parse().map_err(|_| "Config: POSTGRES_PORT not a number")?,
            db_user: env::var("POSTGRES_USER").map_err(|_| "Config: POSTGRES_USER not set")?,
            db_password: env::var("POSTGRES_PASSWORD").map_err(|_| "Config: POSTGRES_PASSWORD not set")?,
            db_name: env::var("POSTGRES_DB").map_err(|_| "Config: POSTGRES_DB not set")?,
            listen_binding: env::var("LISTEN_BINDING").map_err(|_| "Config: LISTEN_BINDING not set")?,
        })
    }
}