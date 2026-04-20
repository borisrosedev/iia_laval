use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
 
/*
CODE MERMAID :
sequenceDiagram
    participant Client
    participant Server
    
    Client->>Server: ClientHello (TLS 1.3, AES-256-GCM, ChaCha20)
    Server->>Client: ServerHello (TLS 1.3, AES-256-GCM)
    Server->>Client: Certificate (CN=example.com)
    Server->>Client: ServerKeyExchange (clé publique DH)
    Server->>Client: ServerHelloDone
    Client->>Server: ClientKeyExchange (clé publique DH)
    Client->>Server: Finished
    Server->>Client: Finished
    Note over Client,Server: Connexion TLS établie


*/



// Lit une ligne depuis un stream TCP
fn lire(stream: &mut BufReader<TcpStream>) -> String {
    let mut ligne = String::new();
    stream.read_line(&mut ligne).unwrap();
    ligne.trim().to_string()
}

// Envoie une ligne sur un stream TCP
fn envoyer(stream: &mut TcpStream, msg: &str) {
    stream.write_all(format!("{}\n", msg).as_bytes()).unwrap();
}

fn serveur() {
    let listener = TcpListener::bind("127.0.0.1:4433").unwrap();
    println!("[Serveur] En écoute sur 127.0.0.1:4433...\n");

    let (stream, _) = listener.accept().unwrap();
    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);

    // Étape 1: reçoit ClientHello
    let msg = lire(&mut reader);
    println!("[Serveur] Reçoit : {}", msg);

    // Étape 2: envoie ServerHello + certificat + clé publique
    println!("[Serveur] Envoie : ServerHello (TLS 1.3, AES-256-GCM)");
    envoyer(&mut writer, "ServerHello (TLS 1.3, AES-256-GCM)");

    println!("[Serveur] Envoie : Certificate (CN=example.com)");
    envoyer(&mut writer, "Certificate (CN=example.com)");

    println!("[Serveur] Envoie : ServerKeyExchange (clé publique DH)");
    envoyer(&mut writer, "ServerKeyExchange (clé publique DH)");

    println!("[Serveur] Envoie : ServerHelloDone");
    envoyer(&mut writer, "ServerHelloDone");

    // Étape 3: reçoit la clé client + Finished
    let msg = lire(&mut reader);
    println!("[Serveur] Reçoit : {}", msg);
    let msg = lire(&mut reader);
    println!("[Serveur] Reçoit : {}", msg);

    // Étape 4: envoie Finished
    println!("[Serveur] Envoie : Finished (session chiffrée)");
    envoyer(&mut writer, "Finished");

    println!("\n[Serveur] Connexion TLS établie");
}

fn client() {
    thread::sleep(Duration::from_millis(150));

    let mut stream = TcpStream::connect("127.0.0.1:4433").unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    // Étape 1: envoie ClientHello
    println!("[Client]  Envoie : ClientHello (TLS 1.3, AES-256-GCM, ChaCha20)");
    envoyer(&mut stream, "ClientHello (TLS 1.3, AES-256-GCM, ChaCha20)");

    // Étape 2: reçoit ServerHello + certificat + clé + done
    let msg = lire(&mut reader);
    println!("[Client]  Reçoit : {}", msg);
    let msg = lire(&mut reader);
    println!("[Client]  Reçoit : {}", msg);
    let msg = lire(&mut reader);
    println!("[Client]  Reçoit : {}", msg);
    let msg = lire(&mut reader);
    println!("[Client]  Reçoit : {}", msg);

    // Étape 3: vérifie le certificat (simulé), envoie sa clé + Finished
    println!("[Client]  Certificat vérifié (simulé)");
    println!("[Client]  Envoie : ClientKeyExchange (clé publique DH)");
    envoyer(&mut stream, "ClientKeyExchange (clé publique DH)");
    println!("[Client]  Envoie : Finished");
    envoyer(&mut stream, "Finished");

    // Étape 4: reçoit Finished du serveur
    let msg = lire(&mut reader);
    println!("[Client]  Reçoit : {}", msg);

    println!("\n[Client]  Connexion TLS établie ");
}

fn main() {
    println!("Simulation handshake TLS 1.3\n");

    let t = thread::spawn(serveur);
    client();
    t.join().unwrap();
}
