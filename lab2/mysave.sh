#!/bin/env bash
set -euo pipefail

usage() {
    echo -e "USAGE:\n\tmysave [OPTIONS]\n\nOPTIONS:\n\t--src=<dir1>\t\tSource directory [default: .]\n\t--dst=<dir2>\t\tDestination directory [required]"
    exit 1
}

src="."
dst=

while [[ "$#" -gt 0 ]]; do
    if [[ "$1" =~ --src=([^ ]*) ]]; then
        src="${BASH_REMATCH[1]}"
    elif [[ "$1" =~ --dst=([^ ]*) ]]; then
        dst="${BASH_REMATCH[1]}"
    else
        echo "Unknown argument: '$1'"
        usage
    fi

    shift
done

if [[ -z "$dst" ]]; then
    echo "No destination given, destination is required!"
    usage
fi

if [[ ! -d "$src" ]]; then
    echo "Given --src ($src) is not a directory!"
    usage
fi

if [[ -d "$dst" ]]; then
    echo "Given --dst ($dst) is already an existing directory!"
    usage
fi

rsync -r "$src" "$dst"
