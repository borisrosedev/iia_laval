use std::net::TcpStream;
use std::time::Duration;

fn main() {
    let host = "127.0.0.1";
    let timeout = Duration::from_millis(250);

    println!("Scan de {} ...", host);

    for port in 1..=1024 {
        let adresse = format!("{}:{}", host, port);
        if TcpStream::connect_timeout(&adresse.parse().unwrap(), timeout).is_ok() {
            println!("[OUVERT] port {}", port);
        }
    }

    println!("Terminé.");
}
