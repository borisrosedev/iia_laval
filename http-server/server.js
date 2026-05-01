const http = require('http')


const server = http.createServer()

server.on('request', (req, res) => {


    if (req.method == "POST" && req.url == "/api/v1/users") {
        const buffer = []

        req.on('data', (chunk) => {
            console.log('✅ [data] event')
            buffer.push(chunk.toString("utf-8"))    
        })

        req.on('end', () => {
            console.log('[end] event')
            const body = JSON.parse(buffer.toString())
            const email = body.email 
            const password = body.password
            console.log("email: " , email)
            console.log("password: ", password)
            return res.end("user logged in")
        })

    }


    if(req.method == "GET" && req.url == "/" ) {
        const headers = new Headers({ foo: 'boris' });
        res.setHeaders(headers)
        return res.end("Hello World!")
    }


    

})
  




server.listen(3001, () => {
    console.log('Server running at http://localhost:3001')
})