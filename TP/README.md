# README - Dossier TP

## Vue d'ensemble

Le dossier `TP` contient 4 sous-projets autour des réseaux :

- `programme_tcp` : simulation pédagogique du handshake TCP en C
- `programme_tls` : simulation pédagogique du handshake TLS 1.3 en C
- `scan_port` : affichage des ports TCP ouverts en Rust
- `websocket` : mini chat temps réel en Node.js avec Socket.IO et SQLite

Ce README explique comment utiliser chaque programme et signale aussi les points importants observés dans l'etat actuel du dossier.

---

## 1. `programme_tcp`

### Role

Ce programme simule les 3 etapes du handshake TCP :

1. `SYN`
2. `SYN-ACK`
3. `ACK`

Le code affiche les etats du client et du serveur, ainsi que les numeros de sequence et d'acquittement.

### Fichiers utiles

- `programme_tcp/src/main.c`
- `programme_tcp/sequencetcp.md`

### Point important

Le dossier contient un `Cargo.toml`, mais le programme principal est un fichier C (`main.c`) et non un fichier Rust (`main.rs`).
Dans l'etat actuel, ce projet ne se lance donc pas directement avec `cargo run`.

### Comment le lancer

Il faut compiler le fichier C avec un compilateur C.

#### Sous Windows avec `gcc` (MinGW par exemple)

```powershell
cd TP/programme_tcp
gcc src/main.c -o programme_tcp.exe
./programme_tcp.exe
```

#### Sous Linux/macOS avec `gcc`

```bash
cd TP/programme_tcp
gcc src/main.c -o programme_tcp
./programme_tcp
```

### Resultat attendu

Le terminal affiche les 3 messages du handshake TCP, puis une confirmation que la connexion est etablie.

### Remarque

Si tu veux absolument l'executer avec Cargo, il faudra soit :

- convertir le programme en Rust
- ajouter une vraie chaine de compilation C adaptee

---

## 2. `programme_tls`

### Role

Ce programme simule une negociation TLS 1.3 :

- `ClientHello`
- `ServerHello`
- `Certificate`
- `CertificateVerify`
- `Finished`

Le but est pedagogique : il montre la logique d'etablissement d'un canal chiffre, pas une implementation TLS reelle.

### Fichiers utiles

- `programme_tls/src/main.c`
- `programme_tls/sequencetls.md`

### Point important

Comme pour `programme_tcp`, le dossier contient un `Cargo.toml` mais le code principal est en C (`main.c`).
Dans l'etat actuel, `cargo run` n'est pas adapte a ce projet.

### Comment le lancer

Compiler puis executer le fichier C.

#### Sous Windows avec `gcc`

```powershell
cd TP/programme_tls
gcc src/main.c -o programme_tls.exe
./programme_tls.exe
```

#### Sous Linux/macOS avec `gcc`

```bash
cd TP/programme_tls
gcc src/main.c -o programme_tls
./programme_tls
```

### Resultat attendu

Le terminal affiche les etapes principales du handshake TLS 1.3 et termine par un message indiquant que le tunnel TLS est etabli.

### Remarque

Ce programme est une simulation textuelle. Il ne chiffre pas de vraies communications reseau.

---

## 3. `scan_port`

### Role

Ce programme Rust liste les ports TCP ouverts sur la machine locale.

Il utilise la bibliotheque `netstat2` pour recuperer les sockets en ecoute, puis groupe les ports par adresse IP.

### Fichiers utiles

- `scan_port/src/main.rs`
- `scan_port/Cargo.toml`

### Prerequis

- Rust installe
- `cargo` disponible dans le terminal

Verification :

```powershell
cargo --version
```

### Comment le lancer

```powershell
cd TP/scan_port
cargo run
```

### Resultat attendu

Le programme affiche :

- les adresses IP detectees
- les ports TCP en ecoute sur chaque adresse

Exemple de sortie :

```text
Analyse des ports reseau ouverts...

Ports TCP ouverts detectes :

Adresse 127.0.0.1
  Port 3000
  Port 5432
```

### Remarque

Si `cargo` n'est pas reconnu, il faut installer Rust via `rustup` puis rouvrir le terminal.

---

## 4. `websocket`

### Role

Ce projet est une application de chat temps reel avec :

- `Express` pour servir la page web
- `Socket.IO` pour les messages en temps reel
- `SQLite` pour stocker l'historique
- un mode `cluster` pour lancer plusieurs workers

### Fichiers utiles

- `websocket/server.js`
- `websocket/index.html`
- `websocket/package.json`
- `websocket/chat.db`

### Prerequis

- Node.js installe
- `npm` disponible

Verification :

```powershell
node --version
npm --version
```

### Installation

Dans l'etat actuel, `package.json` ne declare pas toutes les dependances utilisees dans `server.js`.

Le code importe aussi :

- `sqlite3`
- `sqlite`
- `@socket.io/cluster-adapter`

Il faut donc installer les dependances avec :

```powershell
cd TP/websocket
npm install
npm install sqlite3 sqlite @socket.io/cluster-adapter
```

### Comment le lancer

#### Methode simple

```powershell
cd TP/websocket
node server.js
```

Le serveur lance plusieurs workers, chacun sur un port different a partir de `3000`.

#### Methode dev

```powershell
cd TP/websocket
npm run dev
```

### Comment l'utiliser

1. Lancer le serveur
2. Ouvrir le navigateur
3. Aller sur une adresse comme :
   - `http://localhost:3000`
   - `http://localhost:3001`
   - `http://localhost:3002`
4. Taper un message dans le champ
5. Ouvrir plusieurs onglets pour tester l'echange en temps reel

### Resultat attendu

- les messages apparaissent instantanement
- les messages sont enregistres dans `chat.db`
- lors d'une reconnexion, l'historique peut etre resynchronise

### Remarques importantes

- `npm run dev` utilise `node --watch`, qui peut echouer selon l'environnement ou les permissions
- si tu veux un lancement plus stable pour le TP, `node server.js` est souvent plus simple
- le projet actuel ecoute sur plusieurs ports, pas seulement `3000`, a cause du mode `cluster`

---

## Resume rapide des commandes

### `programme_tcp`

```powershell
cd TP/programme_tcp
gcc src/main.c -o programme_tcp.exe
./programme_tcp.exe
```

### `programme_tls`

```powershell
cd TP/programme_tls
gcc src/main.c -o programme_tls.exe
./programme_tls.exe
```

### `scan_port`

```powershell
cd TP/scan_port
cargo run
```

### `websocket`

```powershell
cd TP/websocket
npm install
npm install sqlite3 sqlite @socket.io/cluster-adapter
node server.js
```

---

## Analyse generale du dossier

- Le dossier est coherent sur le theme reseau et securite reseau
- `scan_port` est le seul projet directement structure comme un vrai projet Rust executable
- `programme_tcp` et `programme_tls` sont des programmes C ranges dans des dossiers de type Cargo
- `websocket` est presque utilisable tel quel, mais il manque des dependances dans `package.json`

Si tu veux, je peux maintenant faire une deuxieme passe et :

- corriger les `package.json` / dependances du projet `websocket`
- transformer `programme_tcp` et `programme_tls` pour qu'ils se lancent proprement
- remplacer aussi le fichier `TP/websocket/README` par une version plus propre et plus presentable
