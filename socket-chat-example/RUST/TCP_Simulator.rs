use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

/*

CODE MERMAID : 
sequenceDiagram
    participant Client
    participant Server
    Client->>Server: SYN
    Server->>Client: SYN-ACK
    Client->>Server: ACK
    Note over Client,Server: Connexion établie

*/



fn main() {
	let addr = "127.0.0.1:4000";

	// 1) Serveur en écoute
	let server = thread::spawn(move || {
		let listener = TcpListener::bind(addr).expect("Impossible de bind le serveur");
		println!("Serveur en écoute sur {}", addr);

		let (mut stream, client_addr) = listener.accept().expect("accept() a échoué");
		println!("Serveur: client connecté: {}", client_addr);

		let mut buf = [0u8; 128];

		// Reçoit "SYN"
		let n = stream.read(&mut buf).expect("read SYN échoué");
		let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();
		println!("Serveur reçoit: {}", msg);

		// Envoie "SYN-ACK"
		stream.write_all(b"SYN-ACK\n").expect("write SYN-ACK échoué");
		println!("Serveur envoie: SYN-ACK");

		// Reçoit "ACK"
		let n = stream.read(&mut buf).expect("read ACK échoué");
		let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();
		println!("Serveur reçoit: {}", msg);
	});

	// Petite pause pour laisser le serveur démarrer
	thread::sleep(Duration::from_millis(150));

	// 2) Client qui se connecte puis simule le handshake en messages
	let mut client = TcpStream::connect(addr).expect("Impossible de se connecter au serveur");
	println!("Client: connecté au serveur");

	// Envoie "SYN"
	client.write_all(b"SYN\n").expect("write SYN échoué");
	println!("Client envoie: SYN");

	// Reçoit "SYN-ACK"
	let mut buf = [0u8; 128];
	let n = client.read(&mut buf).expect("read SYN-ACK échoué");
	let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();
	println!("Client reçoit: {}", msg);

	// Envoie "ACK"
	client.write_all(b"ACK\n").expect("write ACK échoué");
	println!("Client envoie: ACK");

	server.join().expect("Le thread serveur a paniqué");
	println!("Connexion établie");
}