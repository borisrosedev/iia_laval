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
    int seq;
    int ack;
    int syn;
    int ack_flag;
} TCPPacket;

void afficher_paquet(const char *source, const char *destination, TCPPacket packet) {
    printf("%s -> %s | ", source, destination);
    printf("SEQ=%d, ACK=%d, FLAGS=", packet.seq, packet.ack);

    if (packet.syn) {
        printf("SYN ");
    }
    if (packet.ack_flag) {
        printf("ACK ");
    }
    printf("\n");
}

int main() {
    TCPPacket syn_packet;
    TCPPacket syn_ack_packet;
    TCPPacket ack_packet;

    printf("=== Simulation de la négociation TCP ===\n\n");

    sleep_seconds(1);

    syn_packet.seq = 1000;
    syn_packet.ack = 0;
    syn_packet.syn = 1;
    syn_packet.ack_flag = 0;

    printf("1. Le client demande l'ouverture de connexion\n");
    afficher_paquet("Client", "Serveur", syn_packet);

    sleep_seconds(1);

    syn_ack_packet.seq = 2000;
    syn_ack_packet.ack = syn_packet.seq + 1;
    syn_ack_packet.syn = 1;
    syn_ack_packet.ack_flag = 1;

    printf("\n2. Le serveur accepte et accuse reception\n");
    afficher_paquet("Serveur", "Client", syn_ack_packet);

    sleep_seconds(1);

    ack_packet.seq = syn_ack_packet.ack;
    ack_packet.ack = syn_ack_packet.seq + 1;
    ack_packet.syn = 0;
    ack_packet.ack_flag = 1;

    printf("\n3. Le client confirme la reception\n");
    afficher_paquet("Client", "Serveur", ack_packet);

    sleep_seconds(1);

    printf("\nConnexion TCP etablie avec succes.\n");

    return 0;
}