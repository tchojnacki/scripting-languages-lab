#!/bin/env bash
set -euo pipefail
TAB=$'\t'

# Print top n months by death count for a given country

# Require exactly two arguments
if [[ "$#" -ne 2 ]]; then
    echo "Invalid argument count! Exactly 2 required: country, count."
    exit 1
fi

# Validate second script argument
if [[ ! "$2" =~ ^[[:digit:]]*$ ]]; then
    echo "Second argument must be a non-negative number!"
    exit 1
fi

cat covid.tsv |
    # 6,3,2,5 = countriesAndTerritories,year,month,deaths
    process --delimiter="$TAB" --project=6,3,2,5 --select="$1$TAB" |
    # by columns 2 (year) then 3 (month)
    sort -nk2,2 -nk3,3 |
    # group by 1 (year) and 2 (month), aggregate 3 (deaths)
    aggregate --aggr=avg --column-index=3 --group-indices=1,2 |
    # by column 3 (average deaths) descending
    sort -nrk3 |
    head --lines="$2"
