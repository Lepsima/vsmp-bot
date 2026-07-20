#!/usr/bin/env bash

set -euo pipefail

cargo build --release

docker build -t discord-bot .

docker compose up