#!/bin/env bash
set -euo pipefail

# Validate script argument count
if [[ "$#" -eq 0 ]]; then
    echo "No arguments given! At least 1 required - the file name."
    exit 1
fi

# Get script arguments
filename="$1"
shift
keywords="$@"

# Validate if input file exists
if [[ ! -f "$filename" ]]; then
    echo "Input file $filename does not exist!"
    exit 1
fi

# Display params to user
echo "Searching in file $filename occurrences of: $keywords"

# Run the executable
kodpowrotu $keywords < "$filename"
exit_code="$?"

# Display output to user
if [[ "$exit_code" -gt 0 ]]; then
    echo "Found! Most common word: ${!exit_code}, passed as argument $exit_code."
else
    echo "No keyword was found in the text!"
fi
