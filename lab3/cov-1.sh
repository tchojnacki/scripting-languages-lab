CONTINENT="Africa"
YEAR=2020
MONTH=8

SEPARATOR=","

cat covid.tsv |
    # 10,3,2,4 = continentExp,year,month,cases
    process --delimiter=$'\t' --separator="$SEPARATOR" --project=10,3,2,4 --select="$CONTINENT,$YEAR,$MONTH," |
    # 3 = cases
    aggregate --aggr=sum --separator="$SEPARATOR" --column-index=3
