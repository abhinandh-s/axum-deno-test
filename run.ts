import { serveDir } from "https://deno.land/std@0.224.0/http/file_server.ts";
import init, { MyApp } from './pkg/website.js';

await init();
const app = MyApp.new();

Deno.serve((req) => {
  const url = new URL(req.url);

  // 1. Intercept requests for static files
  if (url.pathname.startsWith("/static")) {
    return serveDir(req, {
      fsRoot: "static",    // The folder in your repo
      urlRoot: "static",   // The prefix in the URL
    });
  }

  // 2. Fallback to your Rust/Askama app
  return app.serve(req);
});
