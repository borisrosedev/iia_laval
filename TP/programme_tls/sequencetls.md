Client->>Serveur: ClientHello (Paramètres proposés par le client)
Serveur->>Client: ServerHelloc(Paramètres choisis par le serveur)
Serveur->>Client: Certificat (Identité du serveur)
Client->>Serveur: Échange de clé
Client->>Serveur: Activation du chiffrement (ChangeCipherSpec)
Client->>Serveur: Message de fin
Serveur->>Client: Activation du chiffrement (ChangeCipherSpec)
Serveur->>Client: Message de fin
Note over Client,Serveur: Session TLS chiffrée établie