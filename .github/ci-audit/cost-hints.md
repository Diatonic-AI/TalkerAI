# CI Cost Hints for Diatonic-AI/TalkerAI

Failed runs (window): 36

## Longest failing jobs (minutes)
-   0.98m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17569970123
-   0.88m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17752576324
-   0.88m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17662737635
-   0.87m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17887877711
-   0.87m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17537951249
-   0.85m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17846771743
-   0.85m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17705251325
-   0.83m  📦 Dependency Vulnerability Scan (rust)  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17902867672
-   0.83m  📜 License Compliance Scan  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17720495393
-   0.83m  🔍 Static Application Security Testing  labels=['ubuntu-latest']  run=https://github.com/Diatonic-AI/TalkerAI/actions/runs/17705251325

## Recommendations
- Add concurrency + cancel-in-progress to long-lived workflows (prevents duplicate runs)
- Add on:push paths filters to skip docs-only or non-code changes
- Consider scheduled workflows cadence (weekly/monthly instead of daily)
- Increase cache hit rates (setup-node/setup-python + actions/cache with lockfiles)
- Timeouts: set step/job-level timeouts to prevent runaway costs
- Reduce matrix size or shard by priority (nightly full matrix, PRs minimal)
