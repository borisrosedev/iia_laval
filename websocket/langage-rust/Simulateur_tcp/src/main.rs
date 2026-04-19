fn simuler_tcp() {
    println!("Commencement échange TCP");

    println!("Client -> Serveur : SYN");
    println!("  - Le client essaye établir une connexion");

    println!("Serveur -> Client : SYN/ACK");
    println!("  - Le serveur accepte la demande");
    println!("  - Il accuse réception du SYN du client");

    println!("Client ->  Serveur : ACK");
    println!("  - Le client accuse réception du SYN du serveur");
    println!("  - La connexion TCP est valide");

    println!("Connexion TCP valide");
}

fn main() {
    simuler_tcp();
}
