#include <stdio.h>
#include <unistd.h>

int main() {
    printf("=== TLS connection simulation ===\n\n");

    // Step 1 : ClientHello
    printf("Client -> Server : ClientHello\n");
    printf("  - TLS version : 1.2\n");
    printf("  - Supported cipher suites\n");
    printf("  - Client random\n\n");
    sleep(1);

    // Step 2 : ServerHello
    printf("Server -> Client : ServerHello\n");
    printf("  - TLS version selected : 1.2\n");
    printf("  - Cipher suite selected\n");
    printf("  - Server random\n\n");
    sleep(1);

    // Step 3 : Certificate
    printf("Server -> Client : Certificate\n");
    printf("  - Sending SSL/TLS certificate\n\n");
    sleep(1);

    // Step 4 : ServerHelloDone
    printf("Server -> Client : ServerHelloDone\n\n");
    sleep(1);

    // Step 5 : Certificate verification
    printf("Client : Verifying certificate...\n");
    printf("Client : Certificate valid\n\n");
    sleep(1);

    // Step 6 : Key exchange
    printf("Client -> Server : ClientKeyExchange\n");
    printf("  - Sending premaster secret (encrypted)\n\n");
    sleep(1);

    // Step 7 : Session keys
    printf("Client & Server : Generating session keys\n\n");
    sleep(1);

    // Step 8 : ChangeCipherSpec
    printf("Client -> Server : ChangeCipherSpec\n");
    printf("Client -> Server : Finished (encrypted)\n\n");
    sleep(1);

    printf("Server -> Client : ChangeCipherSpec\n");
    printf("Server -> Client : Finished (encrypted)\n\n");
    sleep(1);

    printf("=== TLS connection established (secure channel) ===\n\n");

    // Secure exchange
    printf("=== Secure data exchange ===\n");
    printf("Client -> Server : \"Hello\" (encrypted)\n");
    sleep(1);

    printf("Server -> Client : \"Hello, client\" (encrypted)\n");
    sleep(1);

    return 0;
}