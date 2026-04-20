#include <stdio.h>
#include <stdlib.h>
#include <winsock2.h>
#include <ws2tcpip.h>

#pragma comment(lib, "ws2_32.lib")

#define TIMEOUT_MS 500

int is_port_open(const char *ip, int port) {
    SOCKET sock;
    struct sockaddr_in target;
    u_long mode = 1;
    fd_set writefds;
    struct timeval tv;
    int result;
    int so_error;
    int len = sizeof(so_error);

    sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (sock == INVALID_SOCKET) {
        return 0;
    }

    if (ioctlsocket(sock, FIONBIO, &mode) != 0) {
        closesocket(sock);
        return 0;
    }

    target.sin_family = AF_INET;
    target.sin_port = htons(port);

    if (inet_pton(AF_INET, ip, &target.sin_addr) != 1) {
        closesocket(sock);
        return 0;
    }

    result = connect(sock, (SOCKADDR *)&target, sizeof(target));

    if (result == SOCKET_ERROR) {
        int err = WSAGetLastError();
        if (err != WSAEWOULDBLOCK && err != WSAEINPROGRESS && err != WSAEINVAL) {
            closesocket(sock);
            return 0;
        }
    }

    FD_ZERO(&writefds);
    FD_SET(sock, &writefds);

    tv.tv_sec = 0;
    tv.tv_usec = TIMEOUT_MS * 1000;

    result = select(0, NULL, &writefds, NULL, &tv);
    if (result > 0 && FD_ISSET(sock, &writefds)) {
        getsockopt(sock, SOL_SOCKET, SO_ERROR, (char *)&so_error, &len);
        closesocket(sock);
        return so_error == 0;
    }

    closesocket(sock);
    return 0;
}

int main() {
    WSADATA wsa;
    char ip[64];
    int startPort, endPort;

    if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0) {
        printf("Erreur : WSAStartup a echoue.\n");
        return 1;
    }

    printf("Entrez l'adresse IP cible : ");
    scanf("%63s", ip);

    printf("Entrez le port de debut : ");
    scanf("%d", &startPort);

    printf("Entrez le port de fin : ");
    scanf("%d", &endPort);

    if (startPort < 1 || endPort > 65535 || startPort > endPort) {
        printf("Plage de ports invalide.\n");
        WSACleanup();
        return 1;
    }

    printf("\nScan des ports TCP de %d a %d sur %s\n\n", startPort, endPort, ip);

    for (int port = startPort; port <= endPort; port++) {
        if (is_port_open(ip, port)) {
            printf("Port %d ouvert\n", port);
        }
    }

    WSACleanup();
    return 0;
}