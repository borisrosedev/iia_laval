#[allow(dead_code)]
#[derive(Debug, Clone)]
enum TlsMessage {
    ClientHello { version: String, cipher_suite: String },
    ServerHello { certificate: String, public_key: String },
    ClientKeyExchange { encrypted_secret: String },
    Finished { encrypted_verification: String },
}

fn main() {
    println!("--- Début de la Simulation TLS Handshake ---\n");


    let client_msg = TlsMessage::ClientHello {
        version: String::from("TLS 1.3"),
        cipher_suite: String::from("AES_256_GCM"),
    };
    println!("[CLIENT] -> Envoie ClientHello : {:?}", client_msg);

 
    let server_msg = simulate_server_hello(client_msg);
    println!("[SERVEUR] -> Envoie ServerHello & Certificat : {:?}", server_msg);


    let key_exchange = simulate_client_key_exchange(server_msg);
    println!("[CLIENT] -> Envoie ClientKeyExchange : {:?}", key_exchange);


    //Crée réellement le message Finished pour supprimer le dernier warning
    let final_msg = TlsMessage::Finished {
        encrypted_verification: String::from("HANDSHAKE_OK_HASH"),
    };
    
    println!("\n[SYSTÈME] -> Calcul des clés de session terminé.");
    println!("[SERVEUR] -> Envoie Finished : {:?}", final_msg);

    println!("\n--- Connexion TLS Établie (Chiffrement Actif) ---");
}

//Simule la réponse du serveur au ClientHello
fn simulate_server_hello(incoming: TlsMessage) -> TlsMessage {
    // On ajoute un "_" devant version car on ne l'utilise pas dans la logique
    if let TlsMessage::ClientHello { version: _version, .. } = incoming {
        TlsMessage::ServerHello {
            certificate: String::from("Certificat_Signé_Par_CA"),
            public_key: String::from("CLE_PUBLIQUE_SERVEUR_RSA"),
        }
    } else {
        panic!("Erreur : Le serveur attendait un ClientHello !");
    }
}

//Simule la création du secret partagé par le client
fn simulate_client_key_exchange(incoming: TlsMessage) -> TlsMessage {
    if let TlsMessage::ServerHello { .. } = incoming {
        TlsMessage::ClientKeyExchange {
            encrypted_secret: String::from("SECRET_CHIFFRE_AVEC_CLE_PUBLIQUE"),
        }
    } else {
        panic!("Erreur : Le client attendait un ServerHello !");
    }
}