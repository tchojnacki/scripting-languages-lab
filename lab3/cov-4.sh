#!/bin/env bash
set -euo pipefail

# Print top n countries with highest sum death count

# Require exactly one argument
if [[ "$#" -ne 1 ]]; then
    echo "Invalid argument count! Exactly 1 required: count."
    exit 1
fi

# Validate script argument
if [[ ! "$1" =~ ^[[:digit:]]*$ ]]; then
    echo "Argument must be a non-negative number!"
    exit 1
fi

cat covid.tsv |
    # 10,6,5 = continentExp,countriesAndTerritories,deaths
    process --delimiter=$'\t' --project=10,6,5 |
    # by 1 (continent) then 2 (country)
    sort -k1,1 -k2,2 |
    # group by 0 (continent) and 1 (country), aggregate 2 (deaths)
    aggregate --aggr=sum --column-index=2 --group-indices=0,1 |
    # by 3 (sum of deaths) descending
    sort -nrk3 |
    myhead --lines="$1"
