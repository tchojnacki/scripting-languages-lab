#!/bin/env bash
set -euo pipefail

cargo install --path lab1/exit-code
cargo install --path lab1/paths
cargo install --path lab1/show-all
cargo install --path lab1/show-similar
cargo install --path lab2/process
cargo install --path lab2/head-tail
cp lab1/exit-code-search.sh ~/bin/exit-code-search
cp lab2/longest-source-files.sh ~/bin/longest-source-files
