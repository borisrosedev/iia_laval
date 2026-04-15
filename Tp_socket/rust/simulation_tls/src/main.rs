fn simuler_tls() {
    println!("échange TLS");

    println!("Client → Serveur : ClientHello");
    println!("Serveur → Client : ServerHello");
    println!("Serveur → Client : Certificat");
    println!("Client → Serveur : Échange de clé");
    println!("Client → Serveur : Activation du chiffrement");
    println!("Client → Serveur : Message de fin");
    println!("Serveur → Client : Activation du chiffrement");
    println!("Serveur → Client : Message de fin");

    println!("échange TLS est bien chiffrée et terminé");
}

fn main() {
    simuler_tls();
}
