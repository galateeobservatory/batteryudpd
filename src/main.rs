use std::net::UdpSocket;
use batteryudpd::{crc16_tarom4545, config, battery_database};

fn main() {
    let config = config::Config::get().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    /*let mut client = Client::configure()
        .host(&*config.db_host)
        .port(config.db_port)
        .user(&*config.db_user)
        .password(&*config.db_password)
        .dbname(&*config.db_name)
        .connect(NoTls)
        .unwrap();
    let row = client.query_one("SELECT * FROM battery WHERE id = 100", &[]).unwrap();
    let charge: f32 = row.get("charge");
    println!("{:?}", charge);
    println!("{:?}", row);*/
    let battery_database = battery_database::BatteryDatabase::new(&config).unwrap_or_else(|err| {
        eprintln!("Problem connecting to PostgreSQL database: {}", err);
        std::process::exit(1);
    });
    let socket = UdpSocket::bind(&config.listen_binding).unwrap();

    loop {
        let mut buf = [0; 150];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("{:?}", amt);
        println!("{:?}", src);
        println!("{:?}", String::from_utf8_lossy(&buf[..amt]));
        crc16_tarom4545::validate_line(String::from_utf8_lossy(&buf[..amt]).trim()).unwrap();
    }
}