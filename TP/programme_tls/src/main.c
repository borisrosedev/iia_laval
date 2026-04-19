#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* ------------------------------------------------------------------ */
/*  Constantes et types                                                 */
/* ------------------------------------------------------------------ */

#define TAILLE_MAX_CHAMP  128

/* Suites cryptographiques supportées par le client */
static const char *SUITES_CLIENT[] = {
    "TLS_AES_256_GCM_SHA384",
    "TLS_CHACHA20_POLY1305_SHA256",
    NULL
};

/* Suite choisie par le serveur */
#define SUITE_CHOISIE "TLS_AES_256_GCM_SHA384"

/* ------------------------------------------------------------------ */
/*  Structures                                                          */
/* ------------------------------------------------------------------ */

/* Message générique TLS */
typedef struct {
    char type[TAILLE_MAX_CHAMP];    /* Type du message (ex: "ClientHello") */
    char contenu[TAILLE_MAX_CHAMP]; /* Données du message                  */
} MessageTLS;

/* États de la poignée de main TLS */
typedef enum {
    TLS_INIT,
    TLS_HELLO_ENVOYE,
    TLS_HELLO_RECU,
    TLS_CERT_RECU,
    TLS_ECHANGE_CLES,
    TLS_TERMINE,
    TLS_ETABLI
} EtatTLS;

/* Entité TLS (client ou serveur) */
typedef struct {
    const char *identite;
    EtatTLS     etat;
    char        cle_publique[TAILLE_MAX_CHAMP];   /* Clé d'échange (simulée) */
    char        suite_selectionnee[TAILLE_MAX_CHAMP];
} ParticipantTLS;

/* ------------------------------------------------------------------ */
/*  Utilitaires d'affichage                                            */
/* ------------------------------------------------------------------ */

static const char *nom_etat_tls(EtatTLS e) {
    switch (e) {
        case TLS_INIT:         return "INIT";
        case TLS_HELLO_ENVOYE: return "HELLO_ENVOYE";
        case TLS_HELLO_RECU:   return "HELLO_RECU";
        case TLS_CERT_RECU:    return "CERTIFICAT_RECU";
        case TLS_ECHANGE_CLES: return "ECHANGE_CLES";
        case TLS_TERMINE:      return "FINISHED";
        case TLS_ETABLI:       return "ETABLI";
        default:               return "INCONNU";
    }
}

static void afficher_message(const MessageTLS *msg,
                              const char *emetteur,
                              const char *recepteur) {
    printf("  [%s -> %s]\n", emetteur, recepteur);
    printf("    Type    : %s\n", msg->type);
    printf("    Contenu : %s\n", msg->contenu);
}

static void afficher_etat_tls(const ParticipantTLS *p) {
    printf("  Etat %-8s : %s\n", p->identite, nom_etat_tls(p->etat));
}

static void ligne(void) {
    printf("  ------------------------------------------\n");
}

/* ------------------------------------------------------------------ */
/*  Étapes de la négociation TLS 1.3                                   */
/* ------------------------------------------------------------------ */

/*
 * Étape 1 : ClientHello
 * Le client annonce les versions TLS, suites crypto, et sa clé publique
 * pour l'échange Diffie-Hellman.
 */
static MessageTLS etape_client_hello(ParticipantTLS *client) {
    MessageTLS msg;
    snprintf(msg.type,    sizeof(msg.type),    "ClientHello");
    snprintf(msg.contenu, sizeof(msg.contenu),
             "Version:TLS1.3 | Suites:%s,%s | ClePublique:%s",
             SUITES_CLIENT[0], SUITES_CLIENT[1],
             client->cle_publique);

    client->etat = TLS_HELLO_ENVOYE;
    return msg;
}

/*
 * Étape 2 : ServerHello
 * Le serveur confirme la version, choisit une suite, envoie sa clé publique.
 * À partir de là, les messages suivants sont chiffrés.
 */
static MessageTLS etape_server_hello(ParticipantTLS *serveur) {
    MessageTLS msg;
    snprintf(msg.type,    sizeof(msg.type),    "ServerHello");
    snprintf(msg.contenu, sizeof(msg.contenu),
             "Version:TLS1.3 | Suite:%s | ClePublique:%s",
             SUITE_CHOISIE, serveur->cle_publique);

    strncpy(serveur->suite_selectionnee, SUITE_CHOISIE,
            sizeof(serveur->suite_selectionnee) - 1);
    serveur->etat = TLS_HELLO_RECU;
    return msg;
}

/*
 * Étape 3 : Certificate
 * Le serveur présente son certificat X.509 pour prouver son identité.
 */
static MessageTLS etape_certificat(ParticipantTLS *serveur) {
    MessageTLS msg;
    snprintf(msg.type,    sizeof(msg.type),    "Certificate");
    snprintf(msg.contenu, sizeof(msg.contenu),
             "Sujet:serveur.rsx102.local | Emetteur:CA_RSX102 | Validite:2025-2027");
    (void)serveur; /* Le serveur émet ce message, pas d'état changé ici */
    return msg;
}

/*
 * Étape 4 : CertificateVerify
 * Le serveur signe un résumé du handshake avec sa clé privée.
 */
static MessageTLS etape_cert_verify(ParticipantTLS *serveur) {
    MessageTLS msg;
    snprintf(msg.type,    sizeof(msg.type),    "CertificateVerify");
    snprintf(msg.contenu, sizeof(msg.contenu),
             "Algo:ECDSA_SHA384 | Signature:[hash_des_messages_precedents]");
    serveur->etat = TLS_CERT_RECU;
    return msg;
}

/*
 * Étape 5 : Finished (serveur)
 * Le serveur envoie un MAC sur l'ensemble du handshake pour garantir l'intégrité.
 */
static MessageTLS etape_finished_serveur(ParticipantTLS *serveur) {
    MessageTLS msg;
    snprintf(msg.type,    sizeof(msg.type),    "Finished (Serveur)");
    snprintf(msg.contenu, sizeof(msg.contenu),
             "MAC:[HMAC-SHA384 sur tous les messages precedents]");
    serveur->etat = TLS_TERMINE;
    return msg;
}

/*
 * Étape 6 : Finished (client)
 * Le client vérifie le MAC, puis envoie le sien pour confirmer sa partie.
 */
static MessageTLS etape_finished_client(ParticipantTLS *client) {
    MessageTLS msg;
    snprintf(msg.type,    sizeof(msg.type),    "Finished (Client)");
    snprintf(msg.contenu, sizeof(msg.contenu),
             "Verification:OK | MAC:[HMAC-SHA384 cote client]");
    client->etat = TLS_TERMINE;
    return msg;
}

/* ------------------------------------------------------------------ */
/*  Point d'entrée                                                     */
/* ------------------------------------------------------------------ */

int main(void) {
    printf("\n");
    printf("============================================\n");
    printf("  RSX102 — Simulation du Handshake TLS 1.3\n");
    printf("============================================\n\n");
    printf("  Hypothese : la connexion TCP est deja etablie.\n\n");

    /* Initialisation des participants */
    ParticipantTLS client = {
        .identite    = "CLIENT",
        .etat        = TLS_INIT,
        .cle_publique = "X25519:0xABCD1234...",
        .suite_selectionnee = ""
    };

    ParticipantTLS serveur = {
        .identite    = "SERVEUR",
        .etat        = TLS_INIT,
        .cle_publique = "X25519:0xEF567890...",
        .suite_selectionnee = ""
    };

    afficher_etat_tls(&client);
    afficher_etat_tls(&serveur);
    ligne();

    /* ---- ETAPE 1 : ClientHello ---- */
    printf("\n[Etape 1] ClientHello — Le client propose ses capacites\n");
    MessageTLS m1 = etape_client_hello(&client);
    afficher_message(&m1, "CLIENT", "SERVEUR");
    afficher_etat_tls(&client);
    ligne();

    /* ---- ETAPE 2 : ServerHello ---- */
    printf("\n[Etape 2] ServerHello — Le serveur choisit la suite crypto\n");
    MessageTLS m2 = etape_server_hello(&serveur);
    afficher_message(&m2, "SERVEUR", "CLIENT");
    client.etat = TLS_HELLO_RECU;
    printf("  >> A partir de maintenant, les messages sont chiffres <<\n");
    afficher_etat_tls(&serveur);
    ligne();

    /* ---- ETAPE 3 : Certificate ---- */
    printf("\n[Etape 3] Certificate — Le serveur prouve son identite\n");
    MessageTLS m3 = etape_certificat(&serveur);
    afficher_message(&m3, "SERVEUR", "CLIENT");
    client.etat = TLS_CERT_RECU;
    ligne();

    /* ---- ETAPE 4 : CertificateVerify ---- */
    printf("\n[Etape 4] CertificateVerify — Le serveur signe le handshake\n");
    MessageTLS m4 = etape_cert_verify(&serveur);
    afficher_message(&m4, "SERVEUR", "CLIENT");
    ligne();

    /* ---- ETAPE 5 : Finished serveur ---- */
    printf("\n[Etape 5] Finished (Serveur) — Integrite du handshake garantie\n");
    MessageTLS m5 = etape_finished_serveur(&serveur);
    afficher_message(&m5, "SERVEUR", "CLIENT");
    afficher_etat_tls(&serveur);
    ligne();

    /* ---- ETAPE 6 : Finished client ---- */
    printf("\n[Etape 6] Finished (Client) — Le client confirme et valide tout\n");
    MessageTLS m6 = etape_finished_client(&client);
    afficher_message(&m6, "CLIENT", "SERVEUR");

    /* Les deux passent à ETABLI */
    client.etat  = TLS_ETABLI;
    serveur.etat = TLS_ETABLI;

    afficher_etat_tls(&client);
    afficher_etat_tls(&serveur);

    printf("\n============================================\n");
    printf("  [SUCCES] Tunnel TLS 1.3 etabli !\n");
    printf("  Toutes les donnees echangees sont chiffrees\n");
    printf("  avec %s.\n", SUITE_CHOISIE);
    printf("============================================\n\n");

    return 0;
}