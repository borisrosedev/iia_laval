const express = require('express')
const dotenv = require('dotenv')
dotenv.config()

const app = express()

app.set('port', process.env.PORT);
app.set('host', process.env.HOST);

app.use(express.json({limit:'10mb'}))
app.get("/", (req, res) => {
    res.status(200).json({message:"C'est Okay"})
})

app.post("/api/v1/users", (req, res) => {
    const body = req.body
    const email = body.email
    const password = body.password
    console.log("Serveur log: email = "+ email +" password = "+ password)
    return res.status(201).json({message:"user crée"})
})

app.listen(app.get('port'), () =>{
    console.log(`Le serveur tourne sur http://${app.get('host')}:${app.get('port')}`);
})

// Invoke-RestMethod -Uri "http://localhost:3000/api/v1/users" -Method POST -ContentType "application/json" -Body '{ "email":"test@example.com", "password":"monSuperMotDePasse" }'
