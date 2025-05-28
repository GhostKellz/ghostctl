use std::net::TcpStream;
use std::time::Duration;

pub fn check_port(host: &str, port: u16) {
    println!("Ghostcat :: Scanning {}:{}...", host, port);

    match TcpStream::connect_timeout(
        &format!("{}:{}", host, port).parse().unwrap(),
        Duration::from_secs(2),
    ) {
        Ok(_) => println!("✅ Port is open"),
        Err(_) => println!("❌ Port is closed or unreachable"),
    }
}
