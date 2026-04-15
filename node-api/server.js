const express = require('express')
const dotenv = require('dotenv')
dotenv.config()
//console.log(process.env)

const app = express()

app.set('port', process.env.PORT);
app.set('host', process.env.HOST);

app.use(express.json({limit: '10mb'}))
app.get("/", (req, res) =>{
    res.status(200).json({message: "All good"})
})

app.listen(app.get(`port`), () => {
    console.log(`Server runs at http://${app.get('host')}:${app.get('port')}`)
})

