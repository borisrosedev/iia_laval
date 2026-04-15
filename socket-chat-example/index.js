const express = require('express');
const { createServer } = require('node:http');
const { join } = require('node:path');
const { Server } = require('socket.io');
const sqlite3 = require('sqlite3');
const { open } = require('sqlite');

async function main() {
  // Ouvrir la base de données
  const db = await open({
    filename: 'chat.db',
    driver: sqlite3.Database
  });

  // Création de la table "messages" si elle n'existe pas déjà
  await db.exec(`
    CREATE TABLE IF NOT EXISTS messages (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        client_offset TEXT UNIQUE,
        content TEXT
    );
  `);

  // Configurer le serveur Express et Socket.IO
  const app = express();
  const server = createServer(app);
  const io = new Server(server, {
    connectionStateRecovery: {}
  });

  // Servir le fichier HTML de base
  app.get('/', (req, res) => {
    res.sendFile(join(__dirname, 'index.html'));
  });

  // Gérer les connexions Socket.IO
  io.on('connection', async(socket) => {
    socket.on('chat message', async (msg, clientOffset, callback) => {
      let result;
      try {
        // Stockage du message dans la base de données
        result = await db.run('INSERT INTO messages (content, client_offset) VALUES (?, ?)', msg, clientOffset);
      } catch (e) {
        if (e.errno === 19 /* SQLITE_CONSTRAINT */ ) {
            // the message was already inserted, so we notify the client
            callback();
        } else {
            // nothing to do, just let the client retry
        }
        return;
      }
      // Inclure l'ID du message dans l'événement émis aux clients
      io.emit('chat message', msg, result.lastID);
        // acknowledge the event
        callback();
    });
    if (!socket.recovered) {
    // Récupérer les messages depuis la base de données et les envoyer au client
    try {
      await db.each('SELECT id, content FROM messages WHERE id > ?',
        [socket.handshake.auth.serverOffset || 0],
        (_err, row) => {
          socket.emit('chat message', row.content, row.id);
        }
      )
    } catch (e) {
         console.error('Erreur lors de la récupération des messages depuis la base de données.', e);
    }
  }
  });

  

  // Démarrer le serveur
  server.listen(3000, () => {
    console.log('server running at http://localhost:3000');
  });
}

main();