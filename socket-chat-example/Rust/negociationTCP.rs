use std::fmt;

// 1. On définit ce qu'est un "Segment TCP" (un morceau de message).
// C'est comme une fiche avec des cases à cocher et des numéros.
struct TcpSegment {
    seq: u32,       // "Sequence" : Le numéro de mon message actuel.
    ack: u32,       // "Acknowledgment" : Le numéro du prochain message que j'attends de l'autre.
    syn: bool,      // Une case à cocher pour dire "On commence ?" (Synchronize).
    ack_flag: bool, // Une case pour dire "J'ai bien reçu ton message" (Acknowledge).
}

// 2. Encore une fois, on explique à Rust comment afficher joliment notre fiche
// quand on utilise println!().
impl fmt::Display for TcpSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Drapeaux:[SYN:{}, ACK:{}] | MonNuméro:{} | J'attendsLe:{}", 
               self.syn, self.ack_flag, self.seq, self.ack)
    }
}

fn main() {
    println!("--- Début du dialogue TCP (Le Handshake) ---");

    // --- ÉTAPE 1 : LE CLIENT VEUT SE CONNECTER ---
    // Je choisis un numéro au hasard pour commencer (ici 1000).
    // Je coche la case SYN pour dire : "Salut, je veux me synchroniser avec toi !"
    let client_initial_seq = 1000;
    let syn_packet = TcpSegment {
        seq: client_initial_seq,
        ack: 0,
        syn: true,
        ack_flag: false,
    };
    println!("Moi -> Serveur : {}", syn_packet);

    // --- ÉTAPE 2 : LE SERVEUR RÉPOND ---
    // Le serveur choisit aussi son numéro (5000).
    // Il coche SYN et ACK pour dire : "Ok pour se synchroniser, j'ai bien reçu ton 1000."
    let server_initial_seq = 5000;
    
    // On vérifie si le serveur a bien reçu notre demande
    let syn_ack_packet = if syn_packet.syn {
        TcpSegment {
            seq: server_initial_seq,
            ack: syn_packet.seq + 1, // Le serveur dit : "J'attends maintenant ton message 1001"
            syn: true,
            ack_flag: true,
        }
    } else {
        panic!("Erreur : Le serveur n'a pas reçu de demande SYN !");
    };
    println!("Serveur -> Moi : {}", syn_ack_packet);

    // --- ÉTAPE 3 : LE CLIENT CONFIRME ---
    // Je vérifie que le serveur attend bien mon message 1001.
    if syn_ack_packet.ack == client_initial_seq + 1 && syn_ack_packet.syn {
        // Je réponds une dernière fois pour confirmer.
        let final_ack = TcpSegment {
            seq: syn_ack_packet.ack,     // J'utilise le numéro 1001 comme prévu.
            ack: syn_ack_packet.seq + 1, // Je dis au serveur : "J'ai reçu ton 5000, j'attends ton 5001"
            syn: false,                  // Plus besoin de SYN, on est déjà d'accord.
            ack_flag: true,              // Je coche ACK pour confirmer la réception.
        };
        println!("Moi -> Serveur : {}", final_ack);
        println!("\n[SUCCÈS] Nous sommes connectés ! On peut maintenant s'envoyer des données.");
    } else {
        println!("\n[ÉCHEC] Les numéros ne correspondent pas, on annule tout.");
    }
}