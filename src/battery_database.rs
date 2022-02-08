use std::cell::RefCell;
use postgres::{Client, NoTls};
use crate::config::Config;

#[allow(dead_code)]
pub struct BatteryDatabase {
    database_client: RefCell<Client>
}

#[allow(dead_code)]
impl BatteryDatabase {
    pub fn new(config: &Config) -> Result<Self, &'static str> {
        Ok(Self {
            database_client: RefCell::new(
                Client::configure()
                    .host(&*config.db_host)
                    .port(config.db_port)
                    .user(&*config.db_user)
                    .password(&*config.db_password)
                    .dbname(&*config.db_name)
                    .connect(NoTls)
                    .map_err(|_| "BatteryDatabase: Failed to connect to database")?
            )
        })
    }
}