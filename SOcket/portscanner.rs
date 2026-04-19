use std::env;
use std::net::TcpStream;
use std::time::Duration;

fn main() {
    // Expect: <program> <ip> <start_port> <end_port>
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <ipv4-address> <start_port> <end_port>", args[0]);
        return;
    }

    let ip = &args[1];
    let start_port: u16 = args[2].parse().expect("Invalid start_port");
    let end_port: u16 = args[3].parse().expect("Invalid end_port");

    if start_port == 0 || end_port > 65535 || start_port > end_port {
        eprintln!("Invalid port range");
        return;
    }

    println!("Scanning {} from {} to {}...", ip, start_port, end_port);

    let timeout = Duration::from_millis(300);

    for port in start_port..=end_port {
        let addr = format!("{}:{}", ip, port);

        // Try to connect with a short timeout
        let result = TcpStream::connect_timeout(
            &addr.parse().expect("Invalid address"),
            timeout,
        );

        if result.is_ok() {
            println!("Port {} is OPEN", port);
        }
        // else: closed/filtered, we stay quiet to keep it simple
    }
}