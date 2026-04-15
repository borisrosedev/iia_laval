const express = require('express')
const dotenv = require('dotenv')
dotenv.config()

const app = express()

app.set('port',process.env.PORT);
app.set('host',process.env.HOST);

app.use(express.json({limit: '10mb'}))
app.get("/", (req, res)=>{
    res.status(200).json({message: "All Good"})
})

app.post("/api/v1/users",(req, res)=> {
    const body = req.body
    const email = req.email
    const password = req.password
    console.log("Server log: email ==>" + email)
    console.log("Server log: password ==>" + password)
    return res.status(201).json({message: "user created"})
})

app.listen(app.get('port'),() => {
    console.log(`Server runs at http://${app.get('host')}:${app.get('port')}`)
})

// console.log(process.env)
