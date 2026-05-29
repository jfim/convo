default:
    @just --list

dev:
    npm run tauri dev

build:
    npm run tauri build

fmt:
    cargo fmt --manifest-path src-tauri/Cargo.toml

lint:
    cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings

test:
    cargo test --manifest-path src-tauri/Cargo.toml

check: fmt lint test
