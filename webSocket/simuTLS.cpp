#include <iostream> 
#include <thread> 
#include <chrono> 
 
void pauseStep() { 
    std::this_thread::sleep_for(std::chrono::milliseconds(700)); 
} 
 
int main() { 
    std::cout << "=== Simulation de la negociation TLS ===\n\n"; 
 
    std::cout << "[CLIENT] Envoi de ClientHello\n"; 
    pauseStep(); 
 
    std::cout << "[SERVEUR] Reception de ClientHello\n"; 
    std::cout << "[SERVEUR] Envoi de ServerHello\n"; 
    std::cout << "[SERVEUR] Envoi du certificat\n"; 
    pauseStep(); 
 
    std::cout << "[CLIENT] Verification du certificat du serveur\n"; 
    pauseStep(); 
 
    std::cout << "[CLIENT] Generation d'un secret de session\n"; 
    std::cout << "[CLIENT] Envoi des informations cryptographiques\n"; 
    pauseStep(); 
 
    std::cout << "[SERVEUR] Calcul du secret partage\n"; 
    pauseStep(); 
 
    std::cout << "[CLIENT] Envoi du message Finished\n"; 
    pauseStep(); 
 
    std::cout << "[SERVEUR] Envoi du message Finished\n"; 
    pauseStep(); 
 
    std::cout << "\nCanal TLS etabli, les echanges peuvent etre chiffres.\n"; 
    return 0; 
}