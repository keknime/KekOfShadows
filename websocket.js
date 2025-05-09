const WebSocket = require('ws');

const wss = new WebSocket.Server({ port: 8080 });
const players = new Map();

wss.on('connection', (ws, req) => {
    const wallet = req.url.slice(1);
    console.log(`Player connected: ${wallet}`);

    ws.on('message', (message) => {
        const data = JSON.parse(message);
        if (data.type === 'update') {
            players.set(wallet, {
                wallet,
                name: data.name,
                level: data.level,
                x: data.x,
                y: data.y,
                location: data.location,
                equipment: data.equipment,
            });

            const playerList = Array.from(players.values());
            wss.clients.forEach((client) => {
                if (client.readyState === WebSocket.OPEN) {
                    client.send(JSON.stringify({ type: 'players', players: playerList }));
                }
            });
        }
    });

    ws.on('close', () => {
        players.delete(wallet);
        const playerList = Array.from(players.values());
        wss.clients.forEach((client) => {
            if (client.readyState === WebSocket.OPEN) {
                client.send(JSON.stringify({ type: 'players', players: playerList }));
            }
        });
        console.log(`Player disconnected: ${wallet}`);
    });
});

console.log('WebSocket server running on ws://localhost:8080');
