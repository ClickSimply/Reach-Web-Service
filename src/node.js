require('http').createServer((req, res) => {
    res.writeHead(200);
    res.end(`<h1>${String(req.url).toLowerCase()}</h1>`);
}).listen(8080);