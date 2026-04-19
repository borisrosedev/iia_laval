#include <iostream> 
#include <string> 
#include <cstring> 
#include <unistd.h> 
#include <arpa/inet.h> 
#include <netinet/in.h> 
#include <sys/socket.h> 
#include <fcntl.h> 
#include <errno.h> 
 
bool isPortOpen(const std::string& ip, int port, int timeoutMs = 300) { 
    int sock = socket(AF_INET, SOCK_STREAM, 0); 
    if (sock < 0) return false; 
 
    int flags = fcntl(sock, F_GETFL, 0); 
    fcntl(sock, F_SETFL, flags | O_NONBLOCK); 
 
    sockaddr_in addr{}; 
    addr.sin_family = AF_INET; 
    addr.sin_port = htons(port); 
 
    if (inet_pton(AF_INET, ip.c_str(), &addr.sin_addr) <= 0) { 
        close(sock); 
        return false; 
    } 
 
    int result = connect(sock, (sockaddr*)&addr, sizeof(addr)); 
    if (result < 0 && errno != EINPROGRESS) { 
        close(sock); 
        return false; 
    } 
 
    fd_set wfds; 
    FD_ZERO(&wfds); 
    FD_SET(sock, &wfds); 
 
    timeval tv{}; 
    tv.tv_sec = timeoutMs / 1000; 
    tv.tv_usec = (timeoutMs % 1000) * 1000; 
 
    result = select(sock + 1, nullptr, &wfds, nullptr, &tv); 
    if (result > 0 && FD_ISSET(sock, &wfds)) { 
        int so_error = 0; 
        socklen_t len = sizeof(so_error); 
        getsockopt(sock, SOL_SOCKET, SO_ERROR, &so_error, &len); 
        close(sock); 
        return so_error == 0; 
    } 
 
    close(sock); 
    return false; 
} 
 
int main() { 
    std::string ip; 
    int startPort, endPort; 
 
    std::cout << "IP cible : "; 
    std::cin >> ip; 
    std::cout << "Port debut : "; 
    std::cin >> startPort; 
    std::cout << "Port fin : "; 
    std::cin >> endPort; 
 
    for (int port = startPort; port <= endPort; ++port) { 
        if (isPortOpen(ip, port)) { 
            std::cout << "Port " << port << " : OUVERT" << std::endl; 
        } else { 
            std::cout << "Port " << port << " : ferme ou filtre" << std::endl; 
        } 
    } 
 
    return 0; 
} 