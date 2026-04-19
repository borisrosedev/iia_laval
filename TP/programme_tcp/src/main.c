#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

/* ------------------------------------------------------------------ */
/*  Structures de données                                               */
/* ------------------------------------------------------------------ */

/*
 * Représentation simplifiée d'un segment TCP.
 * Les drapeaux sont stockés dans un octet de bits.
 */
typedef struct {
    uint32_t num_sequence;   /* Numéro de séquence de l'émetteur      */
    uint32_t num_acquit;     /* Numéro d'acquittement attendu          */
    uint8_t  flag_syn;       /* 1 = demande de synchronisation         */
    uint8_t  flag_ack;       /* 1 = acquittement valide                */
    uint8_t  flag_fin;       /* 1 = demande de fermeture               */
} SegmentTCP;

/* États possibles d'un participant TCP */
typedef enum {
    ETAT_FERME,
    ETAT_ECOUTE,
    ETAT_SYN_ENVOYE,
    ETAT_SYN_RECU,
    ETAT_ETABLI
} EtatTCP;

/* Entité générique (client ou serveur) */
typedef struct {
    const char *nom;
    EtatTCP     etat;
    uint32_t    seq_initial;
    uint32_t    seq_courant;
} ParticipantTCP;

/* ------------------------------------------------------------------ */
/*  Fonctions utilitaires                                               */
/* ------------------------------------------------------------------ */

static const char *nom_etat(EtatTCP e) {
    switch (e) {
        case ETAT_FERME:      return "FERME";
        case ETAT_ECOUTE:     return "ECOUTE";
        case ETAT_SYN_ENVOYE: return "SYN_ENVOYE";
        case ETAT_SYN_RECU:   return "SYN_RECU";
        case ETAT_ETABLI:     return "ETABLI";
        default:              return "INCONNU";
    }
}

static void afficher_segment(const SegmentTCP *seg,
                              const char *expediteur,
                              const char *destinataire) {
    printf("  [%s -> %s]  SEQ=%u | ACK=%u | SYN=%d | ACK_FLAG=%d | FIN=%d\n",
           expediteur, destinataire,
           seg->num_sequence, seg->num_acquit,
           seg->flag_syn, seg->flag_ack, seg->flag_fin);
}

static void afficher_etat(const ParticipantTCP *p) {
    printf("  Etat %-8s : %s\n", p->nom, nom_etat(p->etat));
}

static void separateur(void) {
    printf("  ------------------------------------------\n");
}

/* ------------------------------------------------------------------ */
/*  Logique du handshake                                               */
/* ------------------------------------------------------------------ */

/*
 * Étape 1 : le client envoie SYN
 */
static SegmentTCP client_envoie_syn(ParticipantTCP *client) {
    SegmentTCP seg;
    memset(&seg, 0, sizeof(seg));

    seg.num_sequence = client->seq_initial;
    seg.num_acquit   = 0;
    seg.flag_syn     = 1;
    seg.flag_ack     = 0;

    client->etat       = ETAT_SYN_ENVOYE;
    client->seq_courant = seg.num_sequence;

    return seg;
}

/*
 * Étape 2 : le serveur reçoit SYN et répond SYN-ACK
 */
static SegmentTCP serveur_repond_syn_ack(ParticipantTCP *serveur,
                                          const SegmentTCP *syn_recu) {
    SegmentTCP reponse;
    memset(&reponse, 0, sizeof(reponse));

    /* Vérification du flag SYN */
    if (!syn_recu->flag_syn) {
        fprintf(stderr, "  ERREUR : segment recu sans SYN.\n");
        exit(EXIT_FAILURE);
    }

    reponse.num_sequence = serveur->seq_initial;
    reponse.num_acquit   = syn_recu->num_sequence + 1; /* SEQ client + 1 */
    reponse.flag_syn     = 1;
    reponse.flag_ack     = 1;

    serveur->etat        = ETAT_SYN_RECU;
    serveur->seq_courant = reponse.num_sequence;

    return reponse;
}

/*
 * Étape 3 : le client reçoit SYN-ACK et envoie ACK final
 */
static SegmentTCP client_envoie_ack_final(ParticipantTCP *client,
                                           const SegmentTCP *syn_ack_recu) {
    SegmentTCP ack;
    memset(&ack, 0, sizeof(ack));

    /* Vérification de cohérence : le serveur doit acquitter notre SEQ+1 */
    if (syn_ack_recu->num_acquit != client->seq_initial + 1) {
        fprintf(stderr, "  ERREUR : acquittement incohérent du serveur.\n");
        exit(EXIT_FAILURE);
    }

    ack.num_sequence = syn_ack_recu->num_acquit;          /* = SEQ client + 1 */
    ack.num_acquit   = syn_ack_recu->num_sequence + 1;    /* = SEQ serveur + 1 */
    ack.flag_syn     = 0;
    ack.flag_ack     = 1;

    client->etat       = ETAT_ETABLI;
    client->seq_courant = ack.num_sequence;

    return ack;
}

/* ------------------------------------------------------------------ */
/*  Point d'entrée                                                     */
/* ------------------------------------------------------------------ */

int main(void) {
    printf("\n");
    printf("============================================\n");
    printf("  RSX102 — Simulation du Handshake TCP\n");
    printf("============================================\n\n");

    /* Initialisation des deux participants */
    ParticipantTCP client  = { "CLIENT",  ETAT_FERME,   1000, 0 };
    ParticipantTCP serveur = { "SERVEUR", ETAT_ECOUTE, 7500, 0 };

    afficher_etat(&client);
    afficher_etat(&serveur);
    separateur();

    /* ---- ETAPE 1 : SYN ---- */
    printf("\n[Etape 1] Le client initie la connexion avec SYN\n");
    SegmentTCP seg_syn = client_envoie_syn(&client);
    afficher_segment(&seg_syn, "CLIENT", "SERVEUR");
    afficher_etat(&client);
    separateur();

    /* ---- ETAPE 2 : SYN-ACK ---- */
    printf("\n[Etape 2] Le serveur accepte et repond avec SYN-ACK\n");
    SegmentTCP seg_syn_ack = serveur_repond_syn_ack(&serveur, &seg_syn);
    afficher_segment(&seg_syn_ack, "SERVEUR", "CLIENT");
    afficher_etat(&serveur);
    separateur();

    /* ---- ETAPE 3 : ACK ---- */
    printf("\n[Etape 3] Le client confirme la connexion avec ACK\n");
    SegmentTCP seg_ack = client_envoie_ack_final(&client, &seg_syn_ack);
    afficher_segment(&seg_ack, "CLIENT", "SERVEUR");

    /* Le serveur passe aussi à ETABLI en recevant l'ACK */
    serveur.etat = ETAT_ETABLI;

    afficher_etat(&client);
    afficher_etat(&serveur);

    printf("\n============================================\n");
    printf("  [SUCCES] Connexion TCP etablie !\n");
    printf("  Les deux parties peuvent echanger des donnees.\n");
    printf("============================================\n\n");

    return 0;
}