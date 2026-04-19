sequenceDiagram
    participant Client
    participant Serveur

    Note over Client,Serveur: Prérequis : connexion TCP déjà établie

    Note over Client: Etat: INIT
    Note over Serveur: Etat: INIT

    Client->>Serveur: ClientHello<br/>(TLS 1.3, suites: AES-256-GCM + ChaCha20,<br/>clé publique X25519)
    Note over Client: Etat: HELLO_ENVOYE

    Serveur->>Client: ServerHello<br/>(suite choisie: TLS_AES_256_GCM_SHA384,<br/>clé publique X25519)
    Note over Serveur: Etat: HELLO_RECU

    Note over Client,Serveur: ↓ Tous les messages suivants sont chiffrés ↓

    Serveur->>Client: Certificate<br/>(X.509 : serveur.rsx102.local,<br/>émis par CA_RSX102)
    Note over Client: Etat: CERTIFICAT_RECU

    Serveur->>Client: CertificateVerify<br/>(signature ECDSA-SHA384<br/>du handshake complet)

    Serveur->>Client: Finished<br/>(MAC HMAC-SHA384 du serveur)
    Note over Serveur: Etat: FINISHED

    Client->>Serveur: Finished<br/>(Vérification OK,<br/>MAC HMAC-SHA384 du client)
    Note over Client: Etat: ETABLI
    Note over Serveur: Etat: ETABLI

    Note over Client,Serveur: Tunnel TLS 1.3 sécurisé — données chiffrées avec AES-256-GCM
