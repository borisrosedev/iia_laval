import time

client = "Client"
serveur = "Serveur"

print("=== Simulation d'une connexion TLS ===\n")

# Étape 1 : Client Hello
print(f"{client} → {serveur} : Client Hello (propose une connexion sécurisée)")
time.sleep(1)

# Étape 2 : Server Hello
print(f"{serveur} → {client} : Server Hello (choix des paramètres)")
time.sleep(1)

# Étape 3 : Envoi certificat
print(f"{serveur} → {client} : Certificat (clé publique)")
time.sleep(1)

# Étape 4 : Échange de clé
print(f"{client} → {serveur} : Échange de clé (clé de session)")
time.sleep(1)

# Étape 5 : Connexion sécurisée
print("\nConnexion TLS sécurisée établie")