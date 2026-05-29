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

# Build a release bundle, install the binary to ~/.local/bin, and register the
# convo:// scheme so deep links open the installed (non-dev) app. No sudo needed.
install:
    #!/usr/bin/env bash
    set -euo pipefail
    npm run tauri build
    install -Dm755 src-tauri/target/release/convo "$HOME/.local/bin/convo"
    apps="$HOME/.local/share/applications"
    mkdir -p "$apps"
    printf '%s\n' \
        '[Desktop Entry]' \
        'Type=Application' \
        'Name=convo' \
        "Exec=$HOME/.local/bin/convo %u" \
        'MimeType=x-scheme-handler/convo;' \
        'NoDisplay=true' \
        'Terminal=false' > "$apps/convo.desktop"
    update-desktop-database "$apps" 2>/dev/null || true
    xdg-mime default convo.desktop x-scheme-handler/convo
    echo "Installed convo to ~/.local/bin/convo and registered convo:// scheme."
    echo "Test: xdg-open \"convo://claude-code/<encoded-project>/<uuid>\""
