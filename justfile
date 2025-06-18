# Start Tauri (dev)
tauri-dev:
    cd crates/tauri && cargo tauri dev

# Build the Tauri app for production
tauri-build:
    cd crates/tauri && cargo tauri build

# build crates
build:
    cargo build

install:
    cd apps/desktop && npm install

release:
    cargo build --release

format-backend:
    cargo +nightly fmt

format-frontend:
    cd apps/desktop && npm run format
