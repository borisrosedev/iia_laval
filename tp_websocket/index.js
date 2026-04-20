import express from 'express';
import { createServer } from 'node:http';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { Server } from 'socket.io';

const app = express();
const server = createServer(app);

const io = new Server(server, {
  connectionStateRecovery: {}
});

const __dirname = dirname(fileURLToPath(import.meta.url));

app.get('/', (req, res) => {
  res.sendFile(join(__dirname, 'index.html'));
});

io.on('connection', (socket) => {
  console.log('a user connected');
  socket.on('disconnect', () => {
    console.log('user disconnected');
  });
});

io.on('connection', (socket) => {
  socket.on('chat message', (msg) => {
    console.log('message: ' + msg);
  });
});

io.on('connection', (socket) => {
  socket.on('chat message', (msg) => {
    io.emit('chat message', msg);
  });
});

io.on('connection', (socket) => {
  socket.on('hello', (arg) => {
    console.log(arg);
  });
});

io.on('connection', (socket) => {
  socket.on('hello', (arg1, arg2, arg3) => {
    console.log(arg1); // 1
    console.log(arg2); // '2'
    console.log(arg3); // { 3: '4', 5: <Buffer 06> }
  });
});

io.on('connection', (socket) => {
  socket.on('request', (arg1, arg2, callback) => {
    console.log(arg1); // { foo: 'bar' }
    console.log(arg2); // 'baz'
    callback({
      status: 'ok'
    });
  });
});

socket.onAny((eventName, ...args) => {
  console.log(eventName); // 'hello'
  console.log(args); // [ 1, '2', { 3: '4', 5: ArrayBuffer (1) [ 6 ] } ]
});

socket.onAnyOutgoing((eventName, ...args) => {
  console.log(eventName); // 'hello'
  console.log(args); // [ 1, '2', { 3: '4', 5: ArrayBuffer (1) [ 6 ] } ]
});


io.on('connection', (socket) => {
  // join the room named 'some room'
  socket.join('some room');
  
  // broadcast to all connected clients in the room
  io.to('some room').emit('hello', 'world');

  // broadcast to all connected clients except those in the room
  io.except('some room').emit('hello', 'world');

  // leave the room
  socket.leave('some room');
});
io.emit('hello', 'world');

server.listen(3000, () => {
  console.log('server running at http://localhost:3000');
});