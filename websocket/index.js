const http = require("http");
const ws = require("ws");

const server = http.createServer();

server.on("request", (req, res) => {
	res.writeHead(200);
	res.end("WebSocket server is running");
});

const wss = new ws.WebSocketServer({ server });

let clients = [];
/**@type Map<Number, Set<WebSocket>> */
const rooms = new Map();

wss.on("connection", ws => {
	console.log("New client connected");
	clients.push(ws);

	ws.on("message", message => {
		const data = JSON.parse(message);

		// 加入房间
		const code = data.code;
		const room = rooms.get(code);
		if (data.type == "join") {
			if (!room) {
				rooms.set(code, new Set([ws]));
			} else {
				room.add(ws);
				if (room.size > 1) {
					room.forEach(client => {
						if (client !== ws && client.readyState === ws.OPEN) {
							client.send(JSON.stringify({ type: "join" }));
						}
					});
				}
			}
			return;
		}

		// 广播消息给其他客户端
		if (room)
			room.forEach(client => {
				if (client !== ws && client.readyState === ws.OPEN) {
					client.send(JSON.stringify(data));
				}
			});
	});

	ws.on("close", () => {
		console.log("Client disconnected");
		clients = clients.filter(client => client !== ws);
	});
});

const port = process.env.PORT || 3000;
server.listen(port, "0.0.0.0", () => {
	console.log(`Server is running on port ${port}`);
});
