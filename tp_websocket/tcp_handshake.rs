/// TP RSX102 - Exercice 2 : Simulation d'une négociation TCP (Three-Way Handshake)
///
/// Ce programme simule et affiche les étapes de la négociation TCP :
///   1. Client → Serveur : SYN
///   2. Serveur → Client : SYN-ACK
///   3. Client → Serveur : ACK
///
/// Il établit ensuite une vraie connexion TCP pour démontrer le principe.
///
/// Diagramme UML de séquence (ASCII) :
///
///  Client                    Serveur
///    |                          |
///    |---------- SYN ---------->|   seq=x
///    |                          |
///    |<------ SYN-ACK ----------|   seq=y, ack=x+1
///    |                          |
///    |---------- ACK ---------->|   seq=x+1, ack=y+1
///    |                          |
///    |<=== Connexion établie ===>|
///    |                          |
///    |--- FIN ------------------>|   (fermeture)
///    |<-- FIN-ACK ---------------|
///    |--- ACK ------------------>|

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

/// Représente un paquet TCP simplifié
#[derive(Debug, Clone)]
struct TcpPacket {
    src_port: u16,
    dst_port: u16,
    seq: u32,
    ack: u32,
    flags: TcpFlags,
    payload: Vec<u8>,
}

/// Drapeaux TCP
#[derive(Debug, Clone)]
struct TcpFlags {
    syn: bool,
    ack: bool,
    fin: bool,
    rst: bool,
}

impl TcpFlags {
    fn new() -> Self {
        TcpFlags { syn: false, ack: false, fin: false, rst: false }
    }
    fn to_string(&self) -> String {
        let mut flags = Vec::new();
        if self.syn { flags.push("SYN"); }
        if self.ack { flags.push("ACK"); }
        if self.fin { flags.push("FIN"); }
        if self.rst { flags.push("RST"); }
        flags.join("|")
    }
}

impl TcpPacket {
    fn new(src_port: u16, dst_port: u16, seq: u32, ack: u32, flags: TcpFlags) -> Self {
        TcpPacket { src_port, dst_port, seq, ack, flags, payload: Vec::new() }
    }

    fn display(&self, direction: &str) {
        println!(
            "  {} [{}] seq={} ack={} ({}:{} -> {}:{})",
            direction,
            self.flags.to_string(),
            self.seq,
            self.ack,
            if direction.contains("Client") { "CLIENT" } else { "SERVEUR" },
            self.src_port,
            if direction.contains("Client") { "SERVEUR" } else { "CLIENT" },
            self.dst_port
        );
    }
}

/// Simule la négociation TCP (pédagogique)
fn simulate_tcp_handshake() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║     Simulation Three-Way Handshake TCP       ║");
    println!("╚══════════════════════════════════════════════╝\n");

    println!("  Client                          Serveur");
    println!("    |                                |");

    // Étape 1 : SYN
    thread::sleep(Duration::from_millis(300));
    let client_seq: u32 = 1000;
    let mut flags = TcpFlags::new();
    flags.syn = true;
    let syn = TcpPacket::new(54321, 8080, client_seq, 0, flags);
    println!("    |                                |");
    println!("    |-------- SYN (seq={}) -------->|", syn.seq);
    syn.display("Client → Serveur");

    // Étape 2 : SYN-ACK
    thread::sleep(Duration::from_millis(300));
    let server_seq: u32 = 5000;
    let mut flags2 = TcpFlags::new();
    flags2.syn = true;
    flags2.ack = true;
    let syn_ack = TcpPacket::new(8080, 54321, server_seq, client_seq + 1, flags2);
    println!("    |                                |");
    println!("    |<-- SYN-ACK (seq={}, ack={}) -|", syn_ack.seq, syn_ack.ack);
    syn_ack.display("Serveur → Client");

    // Étape 3 : ACK
    thread::sleep(Duration::from_millis(300));
    let mut flags3 = TcpFlags::new();
    flags3.ack = true;
    let ack = TcpPacket::new(54321, 8080, client_seq + 1, server_seq + 1, flags3);
    println!("    |                                |");
    println!("    |-------- ACK (ack={}) -------->|", ack.ack);
    ack.display("Client → Serveur");

    println!("    |                                |");
    println!("    |<=== CONNEXION ÉTABLIE ========>|");
    println!("    |                                |");

    // Fermeture
    thread::sleep(Duration::from_millis(300));
    println!("    |-------- FIN ----------------->|");
    println!("    |<------- FIN-ACK --------------|");
    println!("    |-------- ACK ----------------->|");
    println!("    |                                |");
    println!("    |<==== CONNEXION FERMÉE ========>|");
}

/// Démonstration avec une vraie connexion TCP
fn real_tcp_demo() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║     Démonstration TCP réelle (port 7878)     ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // Lancer un serveur dans un thread séparé
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Impossible de lier le port 7878");
    println!("[SERVEUR] En écoute sur 127.0.0.1:7878...");

    let server_thread = thread::spawn(move || {
        if let Ok((mut stream, addr)) = listener.accept() {
            println!("[SERVEUR] Connexion acceptée depuis {}", addr);
            println!("[SERVEUR] ✅ Three-way handshake TCP complété par l'OS !");

            let mut buf = [0u8; 512];
            if let Ok(n) = stream.read(&mut buf) {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("[SERVEUR] Message reçu : \"{}\"", msg.trim());
                stream.write_all(b"Bonjour depuis le serveur TCP !").unwrap();
                println!("[SERVEUR] Réponse envoyée.");
            }
        }
    });

    thread::sleep(Duration::from_millis(200));

    // Connexion client
    println!("[CLIENT] Connexion à 127.0.0.1:7878...");
    let mut client = TcpStream::connect("127.0.0.1:7878").expect("Connexion échouée");
    println!("[CLIENT] ✅ Connecté ! (SYN → SYN-ACK → ACK géré par l'OS)");

    client.write_all(b"Bonjour depuis le client TCP !").unwrap();
    println!("[CLIENT] Message envoyé.");

    let mut response = String::new();
    let mut buf = [0u8; 512];
    if let Ok(n) = client.read(&mut buf) {
        response = String::from_utf8_lossy(&buf[..n]).to_string();
    }
    println!("[CLIENT] Réponse reçue : \"{}\"", response);

    server_thread.join().unwrap();
    println!("\n[INFO] Connexion fermée (FIN → FIN-ACK → ACK géré par l'OS)");
}

fn main() {
    simulate_tcp_handshake();
    real_tcp_demo();

    println!("\n\n╔══════════════════════════════════════════════╗");
    println!("║         Diagramme UML de séquence TCP        ║");
    println!("╚══════════════════════════════════════════════╝");
    println!("
  Client                         Serveur
    |                               |
    |------- SYN (seq=x) ---------> |  [CLOSED → SYN_SENT]
    |                               |
    |<-- SYN-ACK (seq=y,ack=x+1) ---|  [LISTEN → SYN_RECEIVED]
    |                               |
    |------- ACK (ack=y+1) -------> |  [ESTABLISHED]
    |                               |
    |<======= DATA EXCHANGE =======>|
    |                               |
    |------- FIN -----------------> |  [FIN_WAIT_1]
    |<------ ACK ------------------|  [CLOSE_WAIT]
    |<------ FIN ------------------|  [LAST_ACK]
    |------- ACK -----------------> |  [TIME_WAIT → CLOSED]
    |                               |
");
}
