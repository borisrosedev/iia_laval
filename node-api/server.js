const express = require('express')
const dotenv = require('dotenv')

dotenv.config()
//console.log(process.env)

const app = express()

app.set('port', process.env.PORT);
app.set('host', process.env.HOST);

app.use(express.json({limit: '10mb'}))

// Requête GET
app.get("/", (req, res) =>{
    res.status(200).json({message: "All good"})
})

// Requête POST
// Commande CURL pour utiliser la requête POST
// curl -X POST http://localhost:3000/api/v1/users \
//      -H "Content-Type: application/json" \
//      -d '{"email": "nicolas@example.com", "password": "ton_password"}'
app.post("/api/v1/users", (req, res) => {
    const body = req.body
    const email = body.email
    const password = body.password
    console.log("Server log: email -> ", + email )
    console.log("Server log: password -> ", + password )
    return res.status(201).json({message: "user created"})
})

// Filtre Wireshark pour observer les SYN, SYN/ACK, ACK
// tcp.flags.syn == 1 or (tcp.seq == 1 and tcp.ack == 1 and tcp.len == 0) 

// Requête LISTEN
app.listen(app.get(`port`), () => {
    console.log(`Server runs at http://${app.get('host')}:${app.get('port')}`)
})
