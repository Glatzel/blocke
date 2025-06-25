#!/usr/bin/env bash
# bench_temp.sh — live Raspberry Pi temperature with stress-ng
#
# Usage:   ./bench_temp.sh [INTERVAL_SECONDS]
# Example: ./bench_temp.sh 2     # prints every 2 s
#          ./bench_temp.sh       # prints every 1 s (default)

set -euo pipefail

INTERVAL="${1:-1}"   # ← default to 1 s if no argument

if ! [[ "$INTERVAL" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
    echo "❌  Interval must be a positive number (seconds)"; exit 1
fi

# ---- check vcgencmd availability ----
if ! command -v vcgencmd >/dev/null; then
  echo "❌  'vcgencmd' not found. Install with: sudo apt install libraspberrypi-bin"; exit 1
fi

get_temp() { vcgencmd measure_temp | cut -d= -f2; }

echo "=== Pi Thermal Monitor (interval: ${INTERVAL}s) ==="
echo "Press Ctrl+C to stop."
echo

stress-ng --cpu 0 --metrics-brief --tz \
          >/tmp/bench_temp_stress.log 2>&1 &
STRESS_PID=$!

finish() {
  echo; echo "Stopping stress-ng..."
  kill -INT "$STRESS_PID" 2>/dev/null || true
  wait "$STRESS_PID" 2>/dev/null || true
  cat /tmp/bench_temp_stress.log
  rm -f /tmp/bench_temp_stress.log
  exit 0
}
trap finish INT TERM

printf "%-8s  %s\n" "Time" "Temp"
while :; do
  printf "%-8s  %s\n" "$(date +%T)" "$(get_temp)"
  sleep "$INTERVAL"
done