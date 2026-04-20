/// TP RSX102 - Exercice 1 : Scanner de ports réseau
/// Vérifie quels ports TCP sont ouverts sur une machine donnée.
///
/// Utilisation : cargo run --bin port_scanner -- <hôte> <port_début> <port_fin>
/// Exemple     : cargo run --bin port_scanner -- 127.0.0.1 1 1024

use std::env;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn scan_port(host: &str, port: u16, timeout_ms: u64) -> bool {
    let addr: SocketAddr = match format!("{}:{}", host, port).parse() {
        Ok(a) => a,
        Err(_) => return false,
    };
    TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)).is_ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <hôte> <port_début> <port_fin>", args[0]);
        eprintln!("Exemple: {} 127.0.0.1 1 1024", args[0]);
        std::process::exit(1);
    }

    let host = args[1].clone();
    let start_port: u16 = args[2].parse().expect("Port de début invalide");
    let end_port: u16 = args[3].parse().expect("Port de fin invalide");
    let timeout_ms: u64 = 200;
    let num_threads: usize = 100;

    println!("=== Scanner de ports ===");
    println!("Cible  : {}", host);
    println!("Plage  : {} - {}", start_port, end_port);
    println!("Timeout: {} ms par port\n", timeout_ms);

    // Vecteur partagé pour stocker les ports ouverts
    let open_ports: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(Vec::new()));
    // File de ports à scanner
    let ports: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(
        (start_port..=end_port).collect(),
    ));

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let host_clone = host.clone();
        let ports_clone = Arc::clone(&ports);
        let open_clone = Arc::clone(&open_ports);

        let handle = thread::spawn(move || loop {
            // Récupérer le prochain port à scanner
            let port = {
                let mut queue = ports_clone.lock().unwrap();
                if queue.is_empty() {
                    break;
                }
                queue.remove(0)
            };

            if scan_port(&host_clone, port, timeout_ms) {
                let mut open = open_clone.lock().unwrap();
                open.push(port);
                println!("[OUVERT] Port {}", port);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut result = open_ports.lock().unwrap();
    result.sort();

    println!("\n=== Résultat ===");
    if result.is_empty() {
        println!("Aucun port ouvert trouvé dans la plage {}-{}.", start_port, end_port);
    } else {
        println!("{} port(s) ouvert(s) : {:?}", result.len(), *result);
    }
}
