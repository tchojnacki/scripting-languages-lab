TAB=$'\t'

COUNTRY="Poland"
COUNT=2

cat covid.tsv |
    # 6,3,2,5 = countriesAndTerritories,year,month,deaths
    process --delimiter="$TAB" --project=6,3,2,5 --select="$COUNTRY$TAB" |
    # by columns 2 (year) then 3 (month)
    sort -nk2,2 -nk3,3 |
    # group by 1 (year) and 2 (month), aggregate 3 (deaths)
    aggregate --aggr=avg --column-index=3 --group-indices=1,2 |
    sort -nrk3 |
    head --lines="$COUNT"
