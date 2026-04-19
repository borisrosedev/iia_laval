fn simuler_tls() {
    println!("échange TLS");

    println!("Client → Serveur : ClientBonjour");
    println!("Serveur → Client : ServerBonjour");
    println!("Serveur → Client : Certificat valide");
    println!("Client → Serveur : Échange key en cours");
    println!("Client → Serveur : Activation du chiffrement");
    println!("Client → Serveur : Message de fin");
    println!("Serveur → Client : Activation du chiffrement");
    println!("Serveur → Client : Message de fin");

    println!("échange TLS est bien chiffrée et terminé");
}

fn main() {
    simuler_tls();
}
