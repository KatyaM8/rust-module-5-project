#!/usr/bin/env bash
set -euo pipefail

mkdir -p artifacts/profiles/latest
cargo build --locked --release --bin profile_workload
./target/release/profile_workload | tee artifacts/profiles/latest/text_profile.csv

if ! command -v perf >/dev/null 2>&1; then
    echo "perf is not installed" >&2
    exit 1
fi

perf record -F 99 --call-graph dwarf \
    -o artifacts/profiles/latest/perf.data \
    -- ./target/release/profile_workload
perf report --stdio -i artifacts/profiles/latest/perf.data \
    > artifacts/profiles/latest/perf_report.txt
