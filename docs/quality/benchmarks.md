# 📈 Benchmark Methodology

No benchmark results are currently published. Do not add speedup or compression-ratio claims without reproducible evidence.

## Dataset classes

Plain text/source code; JSON/CSV/logs; repetitive data; random/high-entropy data; already-compressed files; very small and empty files.

## Metrics

Record input and output bytes, output/input ratio, elapsed wall time, throughput in MiB/s, CPU model, OS, architecture, Rust version, build profile, and compression level.

## Method

1. Use stable synthetic or public datasets with permitted terms.
2. Warm up the binary and filesystem cache consistently.
3. Run multiple repetitions for each input.
4. Report median and spread, not only the best run.
5. Keep datasets and scripts stable across comparisons.
6. Separate filesystem cache effects from compression time where possible.

A dedicated Criterion or custom harness can be added when behavior stabilizes. Avoid benchmark dependencies solely for decorative charts.
