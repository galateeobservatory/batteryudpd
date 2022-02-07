use std::env;
use dotenv::dotenv;
use postgres::{Client, NoTls};

fn main() {
    dotenv().ok();
    let mut client = Client::configure()
        .host(&env::var("POSTGRES_HOST").unwrap())
        .port((&env::var("POSTGRES_PORT").unwrap()).parse().unwrap())
        .user(&env::var("POSTGRES_USER").unwrap())
        .password(&env::var("POSTGRES_PASSWORD").unwrap())
        .dbname(&env::var("POSTGRES_DB").unwrap())
        .connect(NoTls)
        .unwrap();
    let row = client.query_one("SELECT * FROM battery WHERE id = 100", &[]).unwrap();
    let charge: f32 = row.get("charge");
    println!("{:?}", charge);
    println!("{:?}", row);
}