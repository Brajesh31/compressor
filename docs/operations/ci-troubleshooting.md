# 🧰 CI Troubleshooting

| Failure | Typical reason | Remediation |
|---|---|---|
| rustfmt | Formatting drift | Run cargo fmt --all |
| Clippy | Warning elevated to error | Resolve the lint or document a narrow justified allowance |
| Cargo test | Regression or environment issue | Reproduce locally with cargo test --locked |
| Release build | Compilation/linker issue | Run cargo build --locked --release locally |
| CodeQL | Analysis/build incompatibility | Inspect CodeQL job log and update workflow/toolchain deliberately |
| Dependabot | Version conflict or breaking update | Review Cargo.toml/Cargo.lock diff and run the full validation suite |

Always use the failing job log as evidence. Do not disable a security or quality gate merely to make a workflow green.
