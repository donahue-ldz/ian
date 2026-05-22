#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Codex and GUI-launched macOS shells can miss Homebrew and Cargo paths.
export PATH="/opt/homebrew/bin:/usr/local/bin:${HOME}/.cargo/bin:${PATH}"

cd "$REPO_ROOT"

if ! command -v npm >/dev/null 2>&1; then
  echo "error: npm is required to start Ian desktop." >&2
  echo "Install Node.js/npm or add npm to PATH, then retry." >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: Rust Cargo is required to start the Tauri desktop shell." >&2
  echo "Install Rust or add cargo to PATH, then retry." >&2
  exit 1
fi

if [[ ! -d node_modules ]]; then
  echo "Installing npm dependencies..."
  npm install
fi

echo "Starting Ian desktop via Tauri..."
exec npm run desktop:tauri -- dev "$@"
