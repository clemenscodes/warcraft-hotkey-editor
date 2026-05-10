// Zero-dependency static file server used by playwright webServer in CI/Docker.
// Usage: node server.mjs <dir> [port] [basePath]
import { createServer } from "node:http";
import { createReadStream, existsSync, statSync } from "node:fs";
import { join, extname } from "node:path";

const [dir, port = "8080", basePath = ""] = process.argv.slice(2);

const mime = {
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript",
  ".css": "text/css",
  ".wasm": "application/wasm",
  ".ico": "image/x-icon",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".json": "application/json",
  ".txt": "text/plain",
};

createServer((req, res) => {
  const rawPath = req.url.split("?")[0];
  const urlPath =
    basePath && rawPath.startsWith(basePath) ? rawPath.slice(basePath.length) || "/" : rawPath;
  let file = join(dir, urlPath === "/" ? "index.html" : urlPath);
  if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  if (!existsSync(file)) {
    // SPA fallback: walk up path segments to find the nearest index.html
    const segments = urlPath.split("/").filter(Boolean);
    while (segments.length > 0 && !existsSync(join(dir, ...segments, "index.html"))) segments.pop();
    file = segments.length > 0 ? join(dir, ...segments, "index.html") : join(dir, "index.html");
  }
  res.writeHead(200, { "Content-Type": mime[extname(file)] ?? "application/octet-stream" });
  createReadStream(file).pipe(res);
}).listen(parseInt(port), "127.0.0.1");
