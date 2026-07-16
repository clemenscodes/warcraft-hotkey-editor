import { createServer } from "node:http";
import { readFileSync, existsSync, statSync } from "node:fs";
import { join, extname } from "node:path";
import { gzipSync } from "node:zlib";

const [dir, port = "8123", basePath = ""] = process.argv.slice(2);

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

// Compressible types worth gzipping (wasm/js/css dominate transfer; the ~5.4MB
// wasm shrinks ~3.5x). Images are already compressed — serve them raw.
const compressible = new Set([".wasm", ".js", ".css", ".html", ".json", ".svg", ".txt"]);

// In-memory cache of served bytes, keyed by resolved file path. Under 50%-workers
// e2e the same handful of assets are fetched by 16 isolated browser contexts
// concurrently; without this the single-threaded server re-reads and re-streams
// the multi-MB wasm from disk on every request, which serialized page loads and
// looked like a render/parallel bug. Read + gzip each file once, then serve from
// memory.
const cache = new Map();

function load(file) {
  let entry = cache.get(file);
  if (!entry) {
    const raw = readFileSync(file);
    const ext = extname(file);
    const gzipped = compressible.has(ext) ? gzipSync(raw, { level: 6 }) : null;
    entry = { raw, gzipped, type: mime[ext] ?? "application/octet-stream" };
    cache.set(file, entry);
  }
  return entry;
}

createServer((req, res) => {
  const rawPath = req.url.split("?")[0];
  const urlPath =
    basePath && rawPath.startsWith(basePath) ? rawPath.slice(basePath.length) || "/" : rawPath;
  let file = join(dir, urlPath === "/" ? "index.html" : urlPath);
  if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  if (!existsSync(file)) {
    const segments = urlPath.split("/").filter(Boolean);
    while (segments.length > 0 && !existsSync(join(dir, ...segments, "index.html"))) segments.pop();
    file = segments.length > 0 ? join(dir, ...segments, "index.html") : join(dir, "index.html");
  }

  const entry = load(file);
  const acceptsGzip = (req.headers["accept-encoding"] ?? "").includes("gzip");
  const headers = {
    "Content-Type": entry.type,
    // Hashed asset filenames are immutable; letting a context's own reload hit
    // cache halves per-test wasm transfer. index.html stays revalidatable.
    "Cache-Control": file.endsWith(".html") ? "no-cache" : "public, max-age=31536000, immutable",
  };
  if (entry.gzipped && acceptsGzip) {
    headers["Content-Encoding"] = "gzip";
    headers["Content-Length"] = entry.gzipped.length;
    res.writeHead(200, headers);
    res.end(entry.gzipped);
  } else {
    headers["Content-Length"] = entry.raw.length;
    res.writeHead(200, headers);
    res.end(entry.raw);
  }
}).listen(parseInt(port), "127.0.0.1");
