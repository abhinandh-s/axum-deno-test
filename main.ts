
import { serveFile } from "https://deno.land/std@0.203.0/http/file_server.ts";
import { serve } from "https://deno.land/std@0.203.0/http/server.ts";

// Define the base directory for the `dist` folder
const BASE_DIR = "dist";

async function handler(request: Request): Promise<Response> {
  const url = new URL(request.url);
  let filePath = url.pathname;

  // Default to `index.html` for the root path or non-existent paths
  if (filePath === "/") {
    filePath = "/index.html";
  }

  try {
    console.log(`Serving: ${BASE_DIR}${filePath}`); // Log requested file
    return await serveFile(request, `${BASE_DIR}${filePath}`);
  } catch {
    // Fallback to `index.html` for client-side routing
    try {
      return await serveFile(request, `${BASE_DIR}/index.html`);
    } catch {
      // Return 404 if `index.html` is not found
      return new Response("404 Not Found", { status: 404 });
    }
  }
}

console.log("Starting Deno Deploy server...");
serve(handler);

