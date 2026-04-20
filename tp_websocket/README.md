# TP RSX102 – Rust WebSocket & Réseau

## Compilation
```bash
cargo build
```

## Programmes disponibles

### 1. Scanner de ports
```bash
cargo run --bin port_scanner -- <hôte> <port_début> <port_fin>
# Exemple :
cargo run --bin port_scanner -- 127.0.0.1 1 1024
```

### 2. Simulation Three-Way Handshake TCP
```bash
cargo run --bin tcp_handshake
```
Simule et explique la négociation TCP, puis réalise une vraie connexion. Affiche le diagramme UML.

### 3. Simulation Négociation TLS 1.3
```bash
cargo run --bin tls_handshake
```
Simule ClientHello, ServerHello, échange DH, Certificate, Finished. Implémente SHA-1 et Base64 from scratch. Affiche le diagramme UML TCP vs TLS.

### 4. Serveur WebSocket (RFC 6455)
```bash
cargo run --bin websocket_server
```
Serveur WebSocket from scratch (sans dépendance externe).  
Test depuis la console du navigateur :
```javascript
const ws = new WebSocket("ws://127.0.0.1:8080");
ws.onmessage = e => console.log("Reçu:", e.data);
ws.send("Bonjour Rust !");
```

## Architecture technique
- `src/bin/port_scanner.rs`  → scan multi-threadé (100 threads)
- `src/bin/tcp_handshake.rs` → simulation + vraie connexion TCP
- `src/bin/tls_handshake.rs` → SHA-1 & Base64 from scratch, simulation TLS 1.3
- `src/bin/websocket_server.rs` → implémentation RFC 6455 complète
