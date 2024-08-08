#!/bin/bash

set -euo pipefail

ssh -T pyrex << EOF
cd /home/mac/rs-reminder-bot
git clean -fd
git fetch
git reset --hard origin/main
nix-build default.nix
EOF
# User units didn't work, so you have to SSH in afterward and run:
# sudo systemctl restart rs-reminder
