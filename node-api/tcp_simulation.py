import time

client = "Client"
serveur = "Serveur"

print("=== Simulation d'une connexion TCP ===\n")

print(f"{client} → {serveur} : SYN (demande de connexion)")
time.sleep(1)

print(f"{serveur} → {client} : SYN-ACK (accusé de réception)")
time.sleep(1)

print(f"{client} → {serveur} : ACK (confirmation)")
time.sleep(1)

print("\nConnexion TCP établie")