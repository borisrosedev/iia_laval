#include <stdio.h>
#include <unistd.h> // pour sleep()

int main() {
    printf("=== Simulation de connexion TCP ===\n\n");

    // Étape 1 : Client envoie SYN
    printf("Client -> Serveur : SYN\n");
    sleep(1);

    // Étape 2 : Serveur répond SYN-ACK
    printf("Serveur -> Client : SYN-ACK\n");
    sleep(1);

    // Étape 3 : Client envoie ACK
    printf("Client -> Serveur : ACK\n");
    sleep(1);

    printf("\nConnexion TCP établie !\n");

    // Simulation d'échange de données
    printf("\n=== Échange de données ===\n");
    printf("Client -> Serveur : \"Bonjour\"\n");
    sleep(1);

    printf("Serveur -> Client : \"Bonjour, client\"\n");
    sleep(1);

    // Fermeture de connexion
    printf("\n=== Fermeture de la connexion ===\n");
    printf("Client -> Serveur : FIN\n");
    sleep(1);

    printf("Serveur -> Client : ACK\n");
    sleep(1);

    printf("Serveur -> Client : FIN\n");
    sleep(1);

    printf("Client -> Serveur : ACK\n");
    sleep(1);

    printf("\nConnexion fermée.\n");

    return 0;
}