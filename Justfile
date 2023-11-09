dev: 
    just dev-ionic & just dev-rust

dev-ionic:
    @echo 'Watching...'
    (cd infrastructure/ionic && yarn dev)

dev-rust:
    @echo 'Watching...'
    (cd infrastructure/wasm && cargo watch --watch src -- just build-rust)

build-rust:
    @echo 'Building...'
    (cd infrastructure/wasm && wasm-pack build --target web --out-dir ../ionic/src/wasm && rm ../ionic/src/wasm/.gitignore)


