#!/bin/env bash
set -euo pipefail

# Require exactly one argument
if [[ "$#" -ne 1 ]]; then
    echo "Invalid argument count! Exactly 1 required: file extension (without dot)."
    exit 1
fi

# Helper variables
tab=$'\t'
sum=0
count=0

# Process paths output
for size in $(
    paths -Rs --indent="" |
        grep ".$1$tab" |
        process --delimiter="$tab" --project=1
)
do 
    sum=$((sum + size))
    count=$((count + 1))
done

# Print average
echo "$((sum / count))"

# POC: paths -Rs --indent="" | grep ".toml" | process --delimiter=$'\t' --project=1 | awk '{s+=$1}END{print s/NR}'
