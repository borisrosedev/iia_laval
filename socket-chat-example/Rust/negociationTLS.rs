use std::fmt;

// 1. On crée une sorte de "modèle" (une structure) pour représenter nos messages.
// C'est comme une étiquette : on dit quel est le type du message et ce qu'il contient.
struct TlsRecord {
    content_type: String, // Le type de message (ex: "Bonjour")
    data: String,         // Les détails techniques à l'intérieur
}

// 2. Ce bloc sert juste à dire à Rust : "Hé, quand je te demande d'afficher un message,
// fais-le joliment avec des crochets et des barres verticales."
impl fmt::Display for TlsRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Message TLS] Type: {} | Détails: {}", self.content_type, self.data)
    }
}

fn main() {
    println!("--- Début de la discussion sécurisée (TLS 1.3) ---");
    println!("(On part du principe que la connexion de base entre les deux PC est déjà faite)\n");

    // --- ÉTAPE 1 : LE CLIENT DIT BONJOUR (ClientHello) ---
    // Le client (ton PC) envoie une liste de ses capacités :
    // "Je parle TLS 1.3 et je connais ces méthodes de chiffrement."
    let client_hello = TlsRecord {
        content_type: "Coucou (ClientHello)".to_string(),
        data: "Version:TLS1.3, Chiffrement:AES_256, Clé:Curve25519".to_string(),
    };
    println!("Moi -> Serveur : {}", client_hello);

    // --- ÉTAPE 2 : LE SERVEUR RÉPOND (ServerHello) ---
    // Le serveur choisit une méthode dans la liste du client et dit : 
    // "Ok, on utilise ça, et voici ma part de clé pour le secret."
    let server_hello = TlsRecord {
        content_type: "Réponse (ServerHello)".to_string(),
        data: "Choix:AES_256, Clé:Acceptée".to_string(),
    };
    println!("Serveur -> Moi : {}", server_hello);

    // --- ÉTAPE 3 : LE SERVEUR PROUVE QUI IL EST (Certificate) ---
    // Le serveur montre sa "carte d'identité" (le certificat) pour prouver qu'il n'est pas un pirate.
    let server_auth = TlsRecord {
        content_type: "Identité (Certificat)".to_string(),
        data: "ID:ServeurOfficiel, Signature:Valide".to_string(),
    };
    println!("Serveur -> Moi : {}", server_auth);

    // --- ÉTAPE 4 : TOUT EST PRÊT (Finished) ---
    // Le client vérifie l'identité, calcule le secret final et dit : 
    // "C'est bon pour moi, on peut commencer à discuter en secret !"
    let client_finished = TlsRecord {
        content_type: "Terminé (Finished)".to_string(),
        data: "Vérification:OK".to_string(),
    };
    println!("Moi -> Serveur : {}", client_finished);

    println!("\n[SUCCÈS] Le tunnel est maintenant sécurisé !");
    println!("À partir de maintenant, tout ce qu'on s'envoie est illisible pour les autres.");
}