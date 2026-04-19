const express = require('express')
const dotenv = require('dotenv')
const http = require('http')
const WebSocket = require('ws')

dotenv.config()

const app = express()

app.set('port', process.env.PORT || 3000)
app.set('host', process.env.HOST || 'localhost')

app.use(express.json({ limit: '10mb' }))

// 🔹 Route test API
app.get("/", (req, res) => {
    res.status(200).json({ Message: "All Good" })
})

// 🔹 Route POST users
app.post("/api/v1/users", (req, res) => {
    const body = req.body

    if (!body || !body.email || !body.password) {
        return res.status(400).json({ Message: "Champs manquants" })
    }

    const { email, password } = body

    console.log("server log: email ==> " + email)
    console.log("server log: password ==> " + password)

    return res.status(201).json({ Message: "user created" })
})

// 🔹 Route test WebSocket (IMPORTANT pour Burp)
app.get("/test-ws", (req, res) => {
    res.send(`
        <html>
        <body>
        <h1>Test WebSocket</h1>

        <script>
            const ws = new WebSocket("ws://localhost:3000");

            ws.onopen = () => {
                console.log("connecté")
                ws.send("hello serveur")
            }

            ws.onmessage = (msg) => {
                console.log("réponse :", msg.data)
            }

            ws.onerror = (err) => {
                console.log("Erreur :", err)
            }
        </script>

        Ouvre la console (F12) pour voir les messages
        </body>
        </html>
    `)
})

// 🔹 Création serveur HTTP
const server = http.createServer(app)

// 🔹 WebSocket attaché au serveur HTTP
const wss = new WebSocket.Server({ server })

wss.on('connection', (ws) => {
    console.log("Client WebSocket connecté")

    ws.on('message', (message) => {
        console.log("Message reçu :", message.toString())
        ws.send("Message bien reçu côté serveur")
    })

    ws.send("Bienvenue sur le serveur WebSocket")
})

// 🔹 Lancement serveur
server.listen(app.get('port'), () => {
    console.log(`Server runs at http://${app.get('host')}:${app.get('port')}`)
})
