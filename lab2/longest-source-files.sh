#!/bin/env bash

cd "$1"
paths -R -s | sed 's/^ *//g' | process --select=.rs --delimiter=$'\t\t' --separator=" " | sort -k2 -n -r | myhead --lines="$2"
