const http = require('http')

const serverHandler = (req, res) => {

    if(req.method == "GET" && req.url == "/") {
        const headers = new Headers({ foo: 'boris' });
        res.setHeaders(headers)
        return res.end("Hello World!")
    }

    return res.end("No answer")

}


const server = http.createServer(serverHandler)
server.listen(3001, () => {
    console.log('Server running at http://localhost:3001')
})