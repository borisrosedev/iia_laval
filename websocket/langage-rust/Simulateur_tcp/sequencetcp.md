sequenceDiagram
    participant C as Client
    participant S as Serveur

    C->>S: SYN (seq=x)
    Note right of C: Le client initie la connexion
    S->>C: SYN-ACK (seq=y, ack=x+1)
    Note left of S: Le serveur accepte et envoie son propre SYN
    C->>S: ACK (ack=y+1)
    Note over C,S: Connexion TCP établie — three-way handshake terminé
