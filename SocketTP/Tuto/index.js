const express = require('express');             //STEP-1
const { createServer } = require('node:http');  //STEP-1
const { join } = require('node:path');          //STEP-2
const { Server } = require('socket.io');        //STEP-3

const app = express();                          //STEP-1
const server = createServer(app);               //STEP-1
const io = new Server(server, {
  connectionStateRecovery: {}
});



app.get('/', (req, res) => {                    //STEP-2
  res.sendFile(join(__dirname, 'index.html'));  //STEP-2
});                                             //STEP-2

io.on('connection', (socket) => {               //STEP-3
  console.log('a user connected');              //STEP-3
});                                             //STEP-3

io.on('connection', (socket) => {               //STEP-4
  socket.on('chat message', (msg) => {          //STEP-4
    console.log('message: ' + msg);             //STEP-4
  });
});

// this will emit the event to all connected sockets
io.emit('hello', 'world'); 

io.on('connection', (socket) => {
  socket.broadcast.emit('hi');
});

io.on('connection', (socket) => {
  socket.on('chat message', (msg) => {
    io.emit('chat message', msg);
  });
});

// app.get('/', (req, res) => {                    //STEP-1
//   res.send('<h1>Hello world</h1>');             //STEP-1
// });                                             //STEP-1

server.listen(3001, () => {                     //STEP-1
  console.log('server running at http://localhost:3001');
});



//(server.listen...) This means that:

    //Express initializes app to be a function handler that you can supply to an HTTP server (as seen in line 5).
    //We define a route handler / that gets called when we hit our website home.
    //We make the http server listen on port 3001. (3001 cuz I alr have another server running on PORT=3000)

