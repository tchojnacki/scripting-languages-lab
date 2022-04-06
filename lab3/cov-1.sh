#!/bin/env bash
set -euo pipefail

# Print sum of cases for a given continent, year and month

# Require exactly three arguments
if [[ "$#" -ne 3 ]]; then
    echo "Invalid argument count! Exactly 3 required: continent, year, month."
    exit 1
fi

# Validate second script argument
if [[ ! "$2" =~ ^[[:digit:]]*$ ]]; then
    echo "Second argument must be a non-negative number!"
    exit 1
fi

# Validate third script argument
if [[ ! "$3" =~ ^[[:digit:]]*$ ]]; then
    echo "Third argument must be a non-negative number!"
    exit 1
fi

cat covid.tsv |
    # 10,3,2,4 = continentExp,year,month,cases
    process --delimiter=$'\t' --separator=, --project=10,3,2,4 --select="$1,$2,$3," |
    # 3 = cases
    aggregate --aggr=sum --separator=, --column-index=3
