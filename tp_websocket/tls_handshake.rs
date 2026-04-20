/// TP RSX102 - Exercice 3 : Simulation d'une négociation TLS 1.3
///
/// Ce programme simule les étapes de la négociation TLS 1.3 :
///   1. Client  → Serveur : ClientHello  (versions, cipher suites, clé publique DH)
///   2. Serveur → Client  : ServerHello  (version choisie, cipher, clé publique DH)
///   3. Serveur → Client  : Certificate  (certificat X.509 du serveur)
///   4. Serveur → Client  : Finished     (MAC du transcript)
///   5. Client  → Serveur : Finished     (MAC du transcript)
///   6. <=== Canal chiffré établi ===>
///
/// Diagramme UML de séquence TLS 1.3 :
///
///  Client                              Serveur
///    |                                    |
///    |------- ClientHello (DH pub) ------>|
///    |                                    |
///    |<------ ServerHello (DH pub) -------|
///    |<------ Certificate ----------------|
///    |<------ CertificateVerify ----------|
///    |<------ Finished ------------------|
///    |                                    |
///    |------- Finished ------------------>|
///    |                                    |
///    |<======= Canal TLS Chiffré ========>|

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// Algorithmes supportés (simplifiés)
#[derive(Debug, Clone, PartialEq)]
enum CipherSuite {
    TlsAes128GcmSha256,
    TlsAes256GcmSha384,
    TlsChacha20Poly1305Sha256,
}

impl CipherSuite {
    fn name(&self) -> &str {
        match self {
            CipherSuite::TlsAes128GcmSha256 => "TLS_AES_128_GCM_SHA256",
            CipherSuite::TlsAes256GcmSha384 => "TLS_AES_256_GCM_SHA384",
            CipherSuite::TlsChacha20Poly1305Sha256 => "TLS_CHACHA20_POLY1305_SHA256",
        }
    }
}

/// Simule une clé Diffie-Hellman (simplifiée pour l'illustration)
#[derive(Debug, Clone)]
struct DhKeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl DhKeyPair {
    fn generate(seed: u8) -> Self {
        // Simulation simplifiée — en réalité : X25519 ou P-256
        let private_key = vec![seed; 32];
        let public_key = private_key.iter().map(|b| b.wrapping_add(42)).collect();
        DhKeyPair { public_key, private_key }
    }

    fn display_public(&self) -> String {
        format!("{:02x}{:02x}...{:02x}{:02x}",
            self.public_key[0], self.public_key[1],
            self.public_key[30], self.public_key[31])
    }
}

/// Message ClientHello
#[derive(Debug)]
struct ClientHello {
    tls_version: String,
    random: [u8; 32],
    cipher_suites: Vec<CipherSuite>,
    dh_key: DhKeyPair,
    server_name: String,
}

/// Message ServerHello
#[derive(Debug)]
struct ServerHello {
    tls_version: String,
    random: [u8; 32],
    chosen_cipher: CipherSuite,
    dh_key: DhKeyPair,
}

/// Certificat X.509 simplifié
#[derive(Debug)]
struct Certificate {
    subject: String,
    issuer: String,
    valid_from: String,
    valid_to: String,
    public_key_algo: String,
    signature_algo: String,
}

/// État de la session TLS
struct TlsSession {
    state: String,
    cipher_suite: Option<CipherSuite>,
    session_keys: HashMap<String, Vec<u8>>,
}

impl TlsSession {
    fn new() -> Self {
        TlsSession {
            state: "INITIAL".to_string(),
            cipher_suite: None,
            session_keys: HashMap::new(),
        }
    }
}

fn print_separator() {
    println!("  {}", "-".repeat(50));
}

fn simulate_tls_handshake() {
    println!("\n╔══════════════════════════════════════════════════════╗");
    println!("║      Simulation Négociation TLS 1.3                  ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    let mut session = TlsSession::new();

    // ─── Étape 0 : TCP déjà établi ───────────────────────────────────────────
    println!("  [Prérequis] Connexion TCP déjà établie (Three-Way Handshake)");
    println!("  Client                              Serveur");
    println!("    |                                    |");

    // ─── Étape 1 : ClientHello ────────────────────────────────────────────────
    thread::sleep(Duration::from_millis(400));
    let client_dh = DhKeyPair::generate(0xAB);
    let client_hello = ClientHello {
        tls_version: "TLS 1.3".to_string(),
        random: [0x01u8; 32],
        cipher_suites: vec![
            CipherSuite::TlsAes256GcmSha384,
            CipherSuite::TlsAes128GcmSha256,
            CipherSuite::TlsChacha20Poly1305Sha256,
        ],
        dh_key: client_dh.clone(),
        server_name: "example.com".to_string(),
    };

    println!("    |-------- ClientHello -------------->|");
    println!("    |                                    |");
    print_separator();
    println!("  📤 ClientHello");
    println!("     Version TLS    : {}", client_hello.tls_version);
    println!("     SNI (hostname) : {}", client_hello.server_name);
    println!("     Cipher suites  :");
    for cs in &client_hello.cipher_suites {
        println!("       - {}", cs.name());
    }
    println!("     Clé DH client  : {}", client_dh.display_public());
    println!("     Groupes DH     : x25519, secp256r1");
    print_separator();
    session.state = "CLIENT_HELLO_SENT".to_string();

    // ─── Étape 2 : ServerHello ────────────────────────────────────────────────
    thread::sleep(Duration::from_millis(400));
    let server_dh = DhKeyPair::generate(0xCD);
    let server_hello = ServerHello {
        tls_version: "TLS 1.3".to_string(),
        random: [0x02u8; 32],
        chosen_cipher: CipherSuite::TlsAes256GcmSha384,
        dh_key: server_dh.clone(),
    };

    println!("    |                                    |");
    println!("    |<------- ServerHello ---------------|");
    print_separator();
    println!("  📥 ServerHello");
    println!("     Version TLS    : {}", server_hello.tls_version);
    println!("     Cipher choisi  : {}", server_hello.chosen_cipher.name());
    println!("     Clé DH serveur : {}", server_dh.display_public());
    print_separator();
    session.cipher_suite = Some(server_hello.chosen_cipher);

    // ─── Étape 3 : Dérivation des clés (ECDHE) ────────────────────────────────
    thread::sleep(Duration::from_millis(200));
    // Simulation du secret partagé DH : XOR des clés (simplifié)
    let shared_secret: Vec<u8> = client_dh.private_key.iter()
        .zip(server_dh.public_key.iter())
        .map(|(a, b)| a ^ b)
        .collect();
    let handshake_key: Vec<u8> = shared_secret.iter().map(|b| b.wrapping_add(1)).collect();
    let app_key: Vec<u8> = shared_secret.iter().map(|b| b.wrapping_add(2)).collect();

    session.session_keys.insert("handshake_key".to_string(), handshake_key.clone());
    session.session_keys.insert("app_key".to_string(), app_key.clone());

    println!("    |                                    |");
    println!("    |  [Les deux parties dérivent les    |");
    println!("    |   mêmes clés depuis le secret DH]  |");
    print_separator();
    println!("  🔑 Dérivation des clés (HKDF)");
    println!("     Secret partagé DH : {:02x}{:02x}...{:02x}",
        shared_secret[0], shared_secret[1], shared_secret[31]);
    println!("     Clé de handshake  : {:02x}{:02x}...{:02x}",
        handshake_key[0], handshake_key[1], handshake_key[31]);
    println!("     Clé d'application : {:02x}{:02x}...{:02x}",
        app_key[0], app_key[1], app_key[31]);
    println!("     ⚡ À partir d'ici, le trafic est chiffré !");
    print_separator();

    // ─── Étape 4 : Certificate ────────────────────────────────────────────────
    thread::sleep(Duration::from_millis(400));
    let cert = Certificate {
        subject: "CN=example.com, O=Example Corp, C=FR".to_string(),
        issuer: "CN=Let's Encrypt R3, O=Let's Encrypt, C=US".to_string(),
        valid_from: "2024-01-01".to_string(),
        valid_to: "2025-01-01".to_string(),
        public_key_algo: "EC P-256".to_string(),
        signature_algo: "ecdsa-with-SHA256".to_string(),
    };

    println!("    |                                    |");
    println!("    |<------- Certificate [chiffré] -----|");
    print_separator();
    println!("  📜 Certificate (chiffré avec la clé de handshake)");
    println!("     Sujet     : {}", cert.subject);
    println!("     Émetteur  : {}", cert.issuer);
    println!("     Validité  : {} → {}", cert.valid_from, cert.valid_to);
    println!("     Clé pub   : {}", cert.public_key_algo);
    println!("     Signature : {}", cert.signature_algo);
    print_separator();

    // ─── Étape 5 : CertificateVerify + Finished serveur ──────────────────────
    thread::sleep(Duration::from_millis(300));
    println!("    |                                    |");
    println!("    |<---- CertificateVerify [chiffré] --|");
    println!("    |<------- Finished [chiffré] --------|");
    print_separator();
    println!("  ✅ CertificateVerify + Finished (Serveur)");
    println!("     Signature du transcript du handshake avec la clé privée du serveur.");
    println!("     Le client vérifie la signature → authentification du serveur confirmée.");
    print_separator();

    // ─── Étape 6 : Finished client ────────────────────────────────────────────
    thread::sleep(Duration::from_millis(300));
    println!("    |                                    |");
    println!("    |------- Finished [chiffré] -------->|");
    print_separator();
    println!("  ✅ Finished (Client)");
    println!("     MAC du transcript → confirme que le client a les mêmes clés.");
    print_separator();

    // ─── Canal établi ────────────────────────────────────────────────────────
    thread::sleep(Duration::from_millis(300));
    session.state = "ESTABLISHED".to_string();
    println!("    |                                    |");
    println!("    |<====== Canal TLS Chiffré ==========>|");
    println!("    |                                    |");
    println!("    |  [Application Data chiffrée]  <--> |");
    println!("    |                                    |");

    println!("\n╔══════════════════════════════════════════════════════╗");
    println!("║  Session TLS 1.3 : {}  ║", session.state);
    println!("║  Cipher suite    : {}  ║",
        session.cipher_suite.as_ref().map(|c| c.name()).unwrap_or("N/A"));
    println!("╚══════════════════════════════════════════════════════╝");
}

fn print_uml_diagram() {
    println!("\n\n╔══════════════════════════════════════════════════════╗");
    println!("║         Diagramme UML TLS 1.3 vs TCP                ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!("
  Client                              Serveur
    |                                    |
    |== TCP Three-Way Handshake =========|  ← Couche Transport
    |                                    |
    |------- ClientHello (DH pub_C) ---->|  ← TLS Record Layer
    |        (versions, ciphers, SNI)    |
    |                                    |
    |<------ ServerHello (DH pub_S) -----|  ← Cipher choisi
    |                                    |
    |  [Dérivation Secret DH partagé]    |  ← HKDF (RFC 8446)
    |  [Génération clés de session]      |
    |                                    |
    |<------ Certificate [chiffré] ------|  ← X.509
    |<------ CertificateVerify [chiffré]-|  ← Signature
    |<------ Finished [chiffré] ---------|  ← MAC transcript
    |                                    |
    |------- Finished [chiffré] -------->|  ← MAC transcript
    |                                    |
    |<====== Application Data (TLS) ====>|  ← Données chiffrées
    |                                    |

  Différences TCP vs TLS :
  ┌──────────────────────────────────────────────────────┐
  │ TCP  → Établit un canal FIABLE (3 messages)          │
  │         Gère : ordre, retransmission, flux           │
  │         Ne chiffre PAS les données                   │
  ├──────────────────────────────────────────────────────┤
  │ TLS  → Établit un canal SÉCURISÉ (au-dessus du TCP)  │
  │         Gère : chiffrement, authentification, MAC    │
  │         Nécessite TCP au-dessous                     │
  └──────────────────────────────────────────────────────┘
");
}

fn main() {
    simulate_tls_handshake();
    print_uml_diagram();
}
