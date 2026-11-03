const http = require("http");

const port = Number(process.env.PORT || 8000);

http.createServer((request, response) => {
  response.writeHead(200, { "Content-Type": "text/plain" });
  response.end("frameworkless node ok\n");
}).listen(port, "0.0.0.0");
