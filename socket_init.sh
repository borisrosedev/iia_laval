create_socket_api_boilerplate(){
    cd socket-chat-example
    # Initalise un package.json
    npm init -y 

    # Installer les dépendances nécessaires
    npm install express@4
    npm install socket.io
    npm install sqlite sqlite3
    npm install "@socket.io/cluster-adapter"

    # Créer le fichier index.js
    touch index.js

    # Redémarrer le serveur à chaque modification du fichier index.js
    npm pkg set scripts.dev="node --watch index.js"
}

create_socket_api_boilerplate