#!/usr/bin/env bash
# bench_sd.sh – simple fio benchmark for SD / USB / SSD
#
# Usage: ./bench_sd.sh [SIZE] [RUNTIME] [DEPTH]
#        ./bench_sd.sh          # 512 MiB 30 s depth 4
#        ./bench_sd.sh 1G 60 32 # 1 GiB 60 s depth 32

set -e
TEST_FILE="${TEST_FILE:-/mnt/sdcard/fio_test.bin}"

SIZE="${1:-100M}"
RUNTIME="${2:-10}"
DEPTH="${3:-4}"
ENGINE="io_uring"          # fallback to "libaio" on very old kernels

echo "=== Parameters ==="
echo "File size : $SIZE"
echo "Run time  : ${RUNTIME}s"
echo "Queue depth: $DEPTH"
echo "Engine     : $ENGINE"
echo

run() {
  local NAME=$1 BS=$2 RW=$3
  echo "=== $NAME ($BS blocks) ==="
  fio --name="$NAME" --filename="$TEST_FILE" --size="$SIZE" \
      --bs="$BS" --rw="$RW" \
      --ioengine="$ENGINE" --iodepth="$DEPTH" --direct=1 \
      --runtime="$RUNTIME" --time_based --numjobs=1 \
      --group_reporting
  echo
}

run "Sequential write" 1M write
run "Sequential read"  1M read
run "Random write"     4k randwrite
run "Random read"      4k randread

rm -f "$TEST_FILE"
