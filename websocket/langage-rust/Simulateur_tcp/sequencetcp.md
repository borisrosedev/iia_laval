sequenceDiagram
    participant Client
    participant Serveur

    Client->>Serveur: SYN
    Serveur->>Client: SYN/ACK
    Client->>Serveur: ACK
    Note over Client,Serveur: Connexion TCP établie
