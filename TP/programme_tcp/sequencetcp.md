sequenceDiagram
    participant Client
    participant Serveur

    Note over Client: Etat: FERME
    Note over Serveur: Etat: ECOUTE

    Client->>Serveur: SYN (seq=1000, ack=0, SYN=1)
    Note over Client: Etat: SYN_ENVOYE

    Serveur->>Client: SYN-ACK (seq=7500, ack=1001, SYN=1, ACK=1)
    Note over Serveur: Etat: SYN_RECU

    Client->>Serveur: ACK (seq=1001, ack=7501, ACK=1)
    Note over Client: Etat: ETABLI
    Note over Serveur: Etat: ETABLI

    Note over Client,Serveur: Connexion TCP etablie — echange de donnees possible
