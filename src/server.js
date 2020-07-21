const WebSocket = require('ws');

const wss = new WebSocket.Server({ port: 8080 });

wss.on('connection', function connection(ws) {
  ws.on('message', function incoming(message) {
    console.log('received: %s', message);
  });

  setInterval(() => {
    ws.send('ping');
  }, 1000);
});

// let socket = new WebSocket("ws://localhost:8080");
// socket.onmessage = msg => console.log(msg.data);
