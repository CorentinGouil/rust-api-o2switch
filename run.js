const http = require("http");

const { listen } = require("./api.node");

if (typeof PhusionPassenger !== "undefined") {
  PhusionPassenger.configure({ autoInstall: false });
}

if (typeof PhusionPassenger !== "undefined") {
  const server = http.createServer();

  server.listen("passenger", () => {
    const path = server._pipeName;
    server.close();

    listen(path);
  });
} else {
  listen();
}
