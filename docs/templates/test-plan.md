# 🧪 Test Plan

- Change:
- Risk:
- Rust version:
- OS/architecture:

| Test | Input/setup | Expected result | Evidence |
|---|---|---|---|
| Happy path | | | |
| Invalid argument | | | |
| Filesystem error | | | |
| Large input | | | |
| Regression | | | |

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
```
