CONTINENT="Africa"
YEAR=2020
MONTH=8

cat covid.tsv |
    # 10,3,2,4 = continentExp,year,month,cases
    process --delimiter=$'\t' --separator=, --project=10,3,2,4 --select="$CONTINENT,$YEAR,$MONTH," |
    # 3 = cases
    aggregate --aggr=sum --separator=, --column-index=3
