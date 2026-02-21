import { serveDir } from "https://deno.land/std@0.224.0/http/file_server.ts";
import init, { Website } from './pkg/website.js';

await init();
const website = Website.new();

Deno.serve(async (req) => {
  try {
    const url = new URL(req.url);

    if (url.pathname.startsWith("/static")) {
      return await serveDir(req, { fsRoot: "static", urlRoot: "static" });
    }

    return await website.serve(req);
  } catch (e) {
    console.error("WASM Execution Error:", e);
    return new Response("Internal Server Error (WASM Panic)", { status: 500 });
  }
});
