#!/usr/bin/env bash
set -euo pipefail

if ! command -v valgrind >/dev/null 2>&1; then
    echo "valgrind is not installed" >&2
    exit 1
fi

cargo build --locked --bin demo
valgrind \
    --leak-check=full \
    --show-leak-kinds=all \
    --track-origins=yes \
    --error-exitcode=1 \
    ./target/debug/demo
