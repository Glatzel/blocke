#!/usr/bin/env bash
# bench_temp.sh — live temperature monitor with stress-ng for Raspberry Pi, Orange Pi, DietPi
#
# Usage:   ./bench_temp.sh [INTERVAL_SECONDS]
# Example: ./bench_temp.sh 2     # prints every 2 s
#          ./bench_temp.sh       # prints every 1 s (default)

INTERVAL="${1:-1}"

if ! [[ "$INTERVAL" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
    echo "❌  Interval must be a positive number (seconds)"; exit 1
fi

# Detect temperature command
if command -v vcgencmd >/dev/null; then
    get_temp() {
        vcgencmd measure_temp | cut -d= -f2
    }
elif [[ -f /sys/class/thermal/thermal_zone0/temp ]]; then
    get_temp() {
        local t
        t=$(cat /sys/class/thermal/thermal_zone0/temp)
        # Convert millidegrees to degrees with 1 decimal
        echo "scale=1; $(cat /sys/class/thermal/thermal_zone0/temp)/1000" | bc
    }
else
    echo "❌  No known temperature sensor interface found."
    exit 1
fi

echo "=== Thermal Monitor (interval: ${INTERVAL}s) ==="
echo "Press Ctrl+C to stop."
echo

# Start stress-ng CPU stress with metrics and temperature zones
stress-ng --cpu 0 --metrics-brief --tz >/tmp/bench_temp_stress.log 2>&1 &
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
