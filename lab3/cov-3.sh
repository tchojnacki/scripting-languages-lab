#!/bin/env bash
set -euo pipefail

# Print highest peak of epidemy (by daily cases) for each continent

cat covid.tsv |
    # 10,4 = continentExp,cases
    process --delimiter=$'\t' --project=10,4 |
    # by 1 (continentExp)
    sort -k1 |
    # group by 0 (continent), aggregate 1 (cases)
    aggregate --aggr=max --column-index=1 --group-indices=0 |
    # by 2 (max cases) descending
    sort -nrk2
