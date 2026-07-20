#!/usr/bin/env bash

set -euo pipefail

HOST="AD22@192.168.1.100"

echo "Compiling..."
cargo build --release

echo "Building image..."
docker build -t discord-bot .

echo "Sending image..."

docker save discord-bot | gzip | ssh "$HOST" '
set -e

gunzip | /usr/local/bin/docker load

cd /volume1/docker/discord-bot

/usr/local/bin/docker compose up -d
'

echo "Deploy complete."