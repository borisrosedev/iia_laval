class Process:
    def __init__(self, name):
        self.name = name
        self.clock = 0  # Initialisation à 0
        
# Incrémentation simple pour une action interne.
    def local_event(self):
        self.clock += 1
        print(f"[{self.name}] Local : {self.clock}")

# Incrémente avant l'envoi et retourne l'heure actuelle.
    def send_message(self):
        self.clock += 1
        print(f"[{self.name}] Envoi message (Stamp: {self.clock})")
        return self.clock

# Règle : max(locale, reçue) + 1 pour synchroniser.
    def receive_message(self, timestamp):
        self.clock = max(self.clock, timestamp) + 1
        print(f"[{self.name}] Réception (Reçu: {timestamp}) -> Nouvelle horloge : {self.clock}")



# Initialisation
p1 = Process("P1")
p2 = Process("P2")
p3 = Process("P3")

print("--- Début de la simulation ---")

# P1 réalise un événement local
p1.local_event()

# P1 envoie un message à P2
stamp_p1_to_p2 = p1.send_message()

# P2 reçoit le message
p2.receive_message(stamp_p1_to_p2)

# P2 envoie un message à P3
stamp_p2_to_p3 = p2.send_message()

# P3 reçoit le message
p3.receive_message(stamp_p2_to_p3)

print("--- Fin de la simulation ---")