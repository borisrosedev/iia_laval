#include <stdio.h>
#include <stdlib.h>

#ifdef _WIN32
#include <windows.h>
#define sleep_seconds(x) Sleep((x) * 1000)
#else
#include <unistd.h>
#define sleep_seconds(x) sleep(x)
#endif

typedef struct {
    const char *type;
    const char *details;
} TLSMessage;

void afficher_message(const char *source, const char *destination, TLSMessage msg) {
    printf("%s -> %s | %s", source, destination, msg.type);

    if (msg.details != NULL) {
        printf(" | %s", msg.details);
    }

    printf("\n");
}

int main() {
    TLSMessage clientHello = {
        "ClientHello",
        "Version TLS, suites cryptographiques supportees, nombre aleatoire client"
    };

    TLSMessage serverHello = {
        "ServerHello",
        "Version TLS choisie, suite cryptographique retenue, nombre aleatoire serveur"
    };

    TLSMessage certificate = {
        "Certificate",
        "Envoi du certificat du serveur"
    };

    TLSMessage serverHelloDone = {
        "ServerHelloDone",
        "Le serveur a termine sa partie de negociation initiale"
    };

    TLSMessage clientKeyExchange = {
        "ClientKeyExchange",
        "Envoi des informations permettant d'etablir la cle de session"
    };

    TLSMessage clientChangeCipherSpec = {
        "ChangeCipherSpec",
        "Le client passe en mode chiffre"
    };

    TLSMessage clientFinished = {
        "Finished",
        "Le client confirme que le handshake est termine"
    };

    TLSMessage serverChangeCipherSpec = {
        "ChangeCipherSpec",
        "Le serveur passe en mode chiffre"
    };

    TLSMessage serverFinished = {
        "Finished",
        "Le serveur confirme que le handshake est termine"
    };

    printf("=== Simulation de la negociation TLS ===\n\n");

    sleep_seconds(1);

    printf("1. Initialisation de la negociation TLS\n");
    afficher_message("Client", "Serveur", clientHello);

    sleep_seconds(1);

    printf("\n2. Le serveur choisit les parametres de securite\n");
    afficher_message("Serveur", "Client", serverHello);

    sleep_seconds(1);

    printf("\n3. Le serveur envoie son certificat\n");
    afficher_message("Serveur", "Client", certificate);

    sleep_seconds(1);

    printf("\n4. Le serveur termine sa phase initiale\n");
    afficher_message("Serveur", "Client", serverHelloDone);

    sleep_seconds(1);

    printf("\n5. Le client envoie les informations de cle\n");
    afficher_message("Client", "Serveur", clientKeyExchange);

    sleep_seconds(1);

    printf("\n6. Le client active le chiffrement\n");
    afficher_message("Client", "Serveur", clientChangeCipherSpec);

    sleep_seconds(1);

    printf("\n7. Le client termine le handshake\n");
    afficher_message("Client", "Serveur", clientFinished);

    sleep_seconds(1);

    printf("\n8. Le serveur active le chiffrement\n");
    afficher_message("Serveur", "Client", serverChangeCipherSpec);

    sleep_seconds(1);

    printf("\n9. Le serveur termine le handshake\n");
    afficher_message("Serveur", "Client", serverFinished);

    sleep_seconds(1);

    printf("\nConnexion TLS etablie avec succes.\n");
    printf("Les echanges applicatifs peuvent maintenant etre chiffres.\n");

    printf("\nAppuyez sur Entree pour quitter...");
    getchar();

    return 0;
}