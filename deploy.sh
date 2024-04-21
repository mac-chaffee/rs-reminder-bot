#!/bin/bash

set -euo pipefail

ssh -T pyrex << EOF
cd /home/mac/rs-reminder-bot
git clean -fd
git fetch
git reset --hard origin/main
nix-shell -p cargo --command cargo build --release
EOF
