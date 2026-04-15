const http = require('http')

const serverHandler = (req, res) => {

    req.on('connect', () => {


        if (req.method == "POST" && req.url == "/api/v1/users") {
            const buffer = []

        
            req.on('data', (chunk) => {
                buffer.push(chunk.toString("utf-8"))
            })

            req.on('end', () => {
                console.log(buffer)
                return res.end("end")
            })

            res.end("Post")

        }



        if(req.method == "GET" && req.url == "/" ) {
            const headers = new Headers({ foo: 'boris' });
            res.setHeaders(headers)
            return res.end("Hello World!")
        }



        return res.end("No answer 1")





    })
  

    return res.end("No answer 2")





}


const server = http.createServer(serverHandler)
server.listen(3001, () => {
    console.log('Server running at http://localhost:3001')
})