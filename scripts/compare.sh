#!/usr/bin/env bash
set -euo pipefail

for benchmark in normalize_51k fib_32 dedup_2k; do
    mkdir -p "target/criterion/$benchmark/before"
    cp "artifacts/benchmarks/before/$benchmark/"*.json \
        "target/criterion/$benchmark/before/"
done

cargo bench --locked --bench criterion -- --baseline before
