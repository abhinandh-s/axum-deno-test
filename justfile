dev:
  nix develop

install_deployctl:
  deno install -gArf jsr:@deno/deployctl

deploy:
  deployctl deploy

ship:
   git add -A && git commit -m "migration" && git push

start: 
  deno run --allow-net --allow-read main.ts
  
build:
  wasm-pack build --target web

build_release:
  wasm-pack build --target web --release

dev_serve:
   deno run --allow-net --allow-read --watch=main.ts,static/output.css,pkg/ main.ts

watch_css:
    tailwindcss -i ./static/input.css -o ./static/output.css --watch

watch_wasm:
    cargo watch -i .gitignore -s "wasm-pack build --target web"
