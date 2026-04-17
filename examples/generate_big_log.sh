#!/bin/bash
# Generate a large sample log for testing
OUT="examples/big_sample.log"
echo "" > "$OUT"
LEVELS=("INFO" "WARN" "ERROR" "DEBUG" "TRACE")
for i in $(seq 1 1000); do
    LEVEL=${LEVELS[$((RANDOM % 5))]}
    HOUR=$((RANDOM % 24))
    MIN=$((RANDOM % 60))
    SEC=$((RANDOM % 60))
    printf "2026-04-16 %02d:%02d:%02d %s log message number %d from service-%d\n" \
        "$HOUR" "$MIN" "$SEC" "$LEVEL" "$i" "$((RANDOM % 10))" >> "$OUT"
done
echo "generated $OUT with 1000 lines"
