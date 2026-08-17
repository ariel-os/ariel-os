#!/bin/bash
# Memory usage check for AVR targets
# Verifies that .data + .bss <= 2048 bytes (2KB SRAM for ATmega328P)

set -euo pipefail

BINARY="${1:-build/bin/arduino-uno/cargo/avr-none/release/blinky}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

echo "Checking memory usage for: $BINARY"

# Use avr-size to get memory usage
SIZE_OUTPUT=$(avr-size -A "$BINARY" 2>/dev/null || avr-size "$BINARY")

echo "$SIZE_OUTPUT"

# Parse the output to get .data and .bss sizes
# avr-size -A output format:
# section      size      addr
# .data         123       0x800100
# .bss           45       0x800200
# ...

DATA_SIZE=$(echo "$SIZE_OUTPUT" | awk '/^\.data/ {print $2}')
BSS_SIZE=$(echo "$SIZE_OUTPUT" | awk '/^\.bss/ {print $2}')

# Handle case where sections might not exist
DATA_SIZE=${DATA_SIZE:-0}
BSS_SIZE=${BSS_SIZE:-0}

TOTAL_RAM=$((DATA_SIZE + BSS_SIZE))
MAX_RAM=2048  # 2KB for ATmega328P

echo ".data: ${DATA_SIZE} bytes"
echo ".bss:  ${BSS_SIZE} bytes"
echo "Total RAM (.data + .bss): ${TOTAL_RAM} bytes"
echo "Max RAM: ${MAX_RAM} bytes"

if [ "$TOTAL_RAM" -gt "$MAX_RAM" ]; then
    echo "ERROR: RAM usage (${TOTAL_RAM} bytes) exceeds maximum (${MAX_RAM} bytes)!"
    exit 1
else
    REMAINING=$((MAX_RAM - TOTAL_RAM))
    echo "OK: RAM usage within limits. ${REMAINING} bytes remaining."
    exit 0
fi
