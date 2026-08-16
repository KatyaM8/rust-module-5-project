$ErrorActionPreference = 'Stop'

New-Item -ItemType Directory -Force -Path 'artifacts\profiles\latest' | Out-Null
cargo run --locked --release --bin profile_workload |
    Tee-Object -FilePath 'artifacts\profiles\latest\text_profile.csv'
