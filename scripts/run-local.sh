#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "Creating directories..."
mkdir -p docker/config
mkdir -p docker/data

echo "Compiling..."
cargo build --release

echo "Building image..."
docker build -t discord-bot .

echo "Initializing container..."
docker compose up -d --force-recreate
