#include <iostream> 
#include <thread> 
#include <chrono> 
 
void pauseStep() { 
    std::this_thread::sleep_for(std::chrono::milliseconds(700)); 
} 
 
int main() { 
    std::cout << "=== Simulation de la negociation TCP ===\n\n"; 
 
    std::cout << "[CLIENT] Etat initial : CLOSED\n"; 
    pauseStep(); 
 
    std::cout << "[CLIENT] Envoi d'un segment SYN\n"; 
    std::cout << "[CLIENT] Etat -> SYN-SENT\n"; 
    pauseStep(); 
 
    std::cout << "[SERVEUR] Reception du SYN\n"; 
    std::cout << "[SERVEUR] Envoi d'un segment SYN-ACK\n"; 
    std::cout << "[SERVEUR] Etat -> SYN-RECEIVED\n"; 
    pauseStep(); 
 
    std::cout << "[CLIENT] Reception du SYN-ACK\n"; 
    std::cout << "[CLIENT] Envoi d'un segment ACK\n"; 
    std::cout << "[CLIENT] Etat -> ESTABLISHED\n"; 
    pauseStep(); 
 
    std::cout << "[SERVEUR] Reception du ACK\n"; 
    std::cout << "[SERVEUR] Etat -> ESTABLISHED\n"; 
    pauseStep(); 
 
    std::cout << "\nConnexion TCP etablie.\n"; 
    return 0; 
} 