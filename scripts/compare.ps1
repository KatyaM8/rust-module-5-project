$ErrorActionPreference = 'Stop'

foreach ($benchmark in @('normalize_51k', 'fib_32', 'dedup_2k')) {
    $destination = Join-Path "target\criterion\$benchmark" 'before'
    New-Item -ItemType Directory -Force -Path $destination | Out-Null
    Copy-Item -Force "artifacts\benchmarks\before\$benchmark\*.json" $destination
}

cargo bench --locked --bench criterion -- --baseline before
