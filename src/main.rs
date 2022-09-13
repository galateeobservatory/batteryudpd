use batteryudpd::{battery_database, config, battery_reading_line};
use std::net::UdpSocket;

fn main() {
    let config = config::Config::get().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    let mut battery_database = battery_database::BatteryDatabase::new(&config).unwrap_or_else(|err| {
        eprintln!("Problem connecting to PostgreSQL database: {}", err);
        std::process::exit(1);
    });
    let socket = UdpSocket::bind(&config.listen_binding).unwrap();

    loop {
        let mut buf = [0; 150];
        let (amt, _) = match socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error receiving datagram: {}", e);
                continue;
            }
        };
        let line = match battery_reading_line::BatteryReadingLine::new(String::from_utf8_lossy(&buf[..amt]).trim()) {
            Ok(line) => line,
            Err(err_line) => *err_line
        };
        if let Err(e) = battery_database.insert_line(&line) {
            eprintln!("{} - reconnecting to database", e);
            battery_database = battery_database::BatteryDatabase::new(&config).unwrap_or_else(|err| {
                eprintln!("Problem connecting to PostgreSQL database: {}", err);
                std::process::exit(1);
            });
        }
    }
}