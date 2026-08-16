#!/usr/bin/env bash
set -euo pipefail

sanitizer="${1:-address}"
case "$sanitizer" in
    address)
        test_filter=freed_value_is_not_read_again
        ;;
    thread)
        test_filter=concurrent_increment_does_not_lose_updates
        ;;
    *)
        echo "usage: $0 address|thread" >&2
        exit 2
        ;;
esac

RUSTFLAGS="-Zsanitizer=$sanitizer" \
    cargo +nightly test -Zbuild-std --locked \
    --target x86_64-unknown-linux-gnu \
    --test integration "$test_filter" -- --exact --nocapture
