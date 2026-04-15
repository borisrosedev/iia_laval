#!/bin/bash

create_scoket_api_boilerplate(){
    cd socket-chat-example
    # Initalise les fichiers nécessaires
    touch index.js index.html package.json

    # Installation des différentes biblio
    npm install express@4
    npm install socket.io
    npm install sqlite sqlite3
    npm install "@socket.io/cluster-adapter"
    
    # Mettre à jour 
    npm pkg set scripts.dev="node --watch index.js"
    npm pkg set scripts.dev="node --watch index.html"
}

create_socket_api_boilerplate