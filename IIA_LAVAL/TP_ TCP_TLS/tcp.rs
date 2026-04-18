#[derive(Debug, Clone)]
struct TcpPacket {
    syn: bool,
    ack: bool,
    seq: u32,
    ack_num: u32,
}

fn main() {
    println!("--- Début de la simulation TCP Handshake ---\n");

    //Le client choisit un numéro de séquence aléatoire (ici 100)
    let client_seq = 100;
    let syn_packet = TcpPacket {
        syn: true,
        ack: false,
        seq: client_seq,
        ack_num: 0,
    };
    println!("[CLIENT] Envoie SYN (Seq={})", syn_packet.seq);

    //Le serveur reçoit le SYN et prépare sa réponse
    let server_packet = simulate_server_receive(syn_packet);
    println!("[SERVEUR] Envoie SYN-ACK (Seq={}, Ack={})", server_packet.seq, server_packet.ack_num);

    //Le client reçoit le SYN-ACK et confirme
    let final_ack = simulate_client_final_ack(server_packet);
    println!("[CLIENT] Envoie ACK final (Seq={}, Ack={})", final_ack.seq, final_ack.ack_num);

    println!("\n--- Connexion Établie (ESTABLISHED) ---");
}

//Simule la logique du serveur lors de la réception du premier paquet
fn simulate_server_receive(incoming: TcpPacket) -> TcpPacket {
    //Le serveur vérifie que c'est bien un SYN
    if incoming.syn && !incoming.ack {
        let server_seq = 500; //Le serveur choisit son propre numéro de séquence
        
        TcpPacket {
            syn: true,
            ack: true,
            seq: server_seq,
            //L'ACK du serveur est toujours le SEQ du client + 1
            ack_num: incoming.seq + 1,
        }
    } else {
        panic!("Erreur : Le serveur attendait un paquet SYN !");
    }
}

//Simule la réponse finale du client
fn simulate_client_final_ack(incoming: TcpPacket) -> TcpPacket {
    //Le client vérifie que le serveur a bien acquitté son SYN
    if incoming.syn && incoming.ack {
        TcpPacket {
            syn: false,
            ack: true,
            //Le nouveau SEQ du client est l'ACK envoyé par le serveur
            seq: incoming.ack_num,
            //L'ACK du client est le SEQ du serveur + 1
            ack_num: incoming.seq + 1,
        }
    } else {
        panic!("Erreur : Le client attendait un paquet SYN-ACK !");
    }
}