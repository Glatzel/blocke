#!/usr/bin/env bash
# bench_sd.sh  – simple fio benchmark for SD / USB / SSD
#
# Usage:   ./bench_sd.sh [SIZE] [RUNTIME] [DEPTH]
# Example: ./bench_sd.sh           # defaults: 512M 30s depth 4
#          ./bench_sd.sh 1G 60 8   # 1-GiB test file, 60-s runs, queue-depth 8

set -euo pipefail

# ---- adjust this if your SD card mounts elsewhere ----
TEST_FILE="/mnt/sdcard/fio_test.bin"

SIZE="${1:-512M}"    # default 512 MiB
RUNTIME="${2:-30}"   # default 30 s
DEPTH="${3:-4}"      # default iodepth 4

echo "=== Parameters ==="
echo "File size : $SIZE"
echo "Run time  : ${RUNTIME}s"
echo "Queue depth: $DEPTH"
echo

# --- Sequential write (1 MiB blocks) ---
echo "=== Sequential write (1 MiB blocks) ==="
fio --name=seq_write --filename="$TEST_FILE" --size="$SIZE" \
    --bs=1M --rw=write --iodepth="$DEPTH" --direct=1 \
    --runtime="$RUNTIME" --time_based

# --- Sequential read (1 MiB blocks) ---
echo -e "\n=== Sequential read (1 MiB blocks) ==="
fio --name=seq_read --filename="$TEST_FILE" --size="$SIZE" \
    --bs=1M --rw=read --iodepth="$DEPTH" --direct=1 \
    --runtime="$RUNTIME" --time_based

# --- Random write (4 KiB blocks) ---
echo -e "\n=== Random write (4 KiB blocks) ==="
fio --name=rand_write --filename="$TEST_FILE" --size="$SIZE" \
    --bs=4k --rw=randwrite --iodepth="$DEPTH" --direct=1 \
    --runtime="$RUNTIME" --time_based

# --- Random read (4 KiB blocks) ---
echo -e "\n=== Random read (4 KiB blocks) ==="
fio --name=rand_read --filename="$TEST_FILE" --size="$SIZE" \
    --bs=4k --rw=randread --iodepth="$DEPTH" --direct=1 \
    --runtime="$RUNTIME" --time_based

rm -f "$TEST_FILE"