dev:
  nix develop

install_deployctl:
  deno install -gArf jsr:@deno/deployctl

deploy:
  deployctl deploy

ship:
   git add -A && git commit -m "migration" && git push

start: 
  deno run --allow-net --allow-read run.ts
  
build:
  wasm-pack build --target web --release
