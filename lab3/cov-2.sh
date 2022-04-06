COUNTRY="Poland"
COUNT=2

SEPARATOR=$'\t'

cat covid.tsv |
    # 6,3,2,5 = countriesAndTerritories,year,month,deaths
    process --delimiter=$'\t' --separator="$SEPARATOR" --project=6,3,2,5 --select="$COUNTRY," |
    # by columns 2 (year) then 3 (month)
    sort -t "$SEPARATOR" -k2,2 -k3,3 |
    # group by 1 (year) and 2 (month), aggregate 3 (deaths)
    aggregate --aggr=avg --separator="$SEPARATOR" --column-index=3 --group-indices=1,2 |
    sort -t "$SEPARATOR" -nrk3 |
    head --lines="$COUNT" |
    process --delimiter="$SEPARATOR" --separator=$'\t'
