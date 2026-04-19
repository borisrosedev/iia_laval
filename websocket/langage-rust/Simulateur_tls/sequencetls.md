sequenceDiagram
    participant C as Client
    participant S as Serveur

    C->>S: ClientHello (versions TLS + suites cryptographiques proposées)
    S->>C: ServerHello (version et suite retenues)
    S->>C: Certificat X.509 (identité du serveur)
    C->>S: KeyExchange (clé de session chiffrée)
    C->>S: ChangeCipherSpec (activation du chiffrement côté client)
    C->>S: Finished (hash chiffré du handshake)
    S->>C: ChangeCipherSpec (activation du chiffrement côté serveur)
    S->>C: Finished (hash chiffré du handshake)
    Note over C,S: Session TLS chiffrée et authentifiée établie
