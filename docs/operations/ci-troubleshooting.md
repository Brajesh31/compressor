# 🧰 CI Troubleshooting

| Failure | Typical reason | Remediation |
|---|---|---|
| rustfmt | Formatting drift | Run cargo fmt --all |
| Clippy | Warning elevated to error | Resolve the lint or document a narrow justified allowance |
| Cargo test | Regression or environment issue | Reproduce with cargo test --locked |
| Release build | Compilation/linker issue | Run cargo build --locked --release locally |
| CodeQL | Analysis/build incompatibility | Inspect job logs and update the workflow/toolchain deliberately |
| Dependabot | Version conflict or breaking update | Review Cargo.toml/Cargo.lock diff and run the validation suite |

Use the failing job log as evidence. Do not disable security or quality gates merely to make a workflow green.
