#!/usr/bin/env bash
set -euo pipefail
echo "=== CPU Integer (sysbench) ==="
sysbench cpu --threads="$(nproc)" --time=30 run
