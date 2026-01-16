import init, { MyApp } from './pkg/website.js';

await init()
const app = MyApp.new()
Deno.serve((r) => app.serve(r))

