#!/bin/env bash
set -euo pipefail

# Validate script argument count
if [[ "$#" -ne 2 ]]; then
    echo "Invalid argument count! Exactly 2 required: directory and N."
    exit 1
fi

# Validate first script argument
if [[ ! -d "$1" ]]; then
    echo "First argument must be a valid directory!"
    exit 1
fi

# Validate second script argument
if [[ ! "$2" =~ ^[[:digit:]]*$ ]]; then
    echo "Second argument must be a non-negative number!"
    exit 1
fi

# Paths works from current directory, change to requested dir
cd "$1"

# Process paths output
paths -Rs --indent="" |
    process --select=.rs --delimiter=$'\t' --separator=" " |
    sort -nrk2 |
    myhead --lines="$2"
