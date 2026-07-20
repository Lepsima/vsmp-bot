#!/usr/bin/env bash

HOST="AD22@192.168.1.100"

ssh "$HOST" '
cd /volume1/docker/discord-bot
/usr/local/bin/docker compose restart
'