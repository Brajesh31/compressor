# 🧪 Testing Strategy

## Unit coverage

Tests cover successful Gzip compression followed by decompression and byte comparison, empty source files, identical source/target path rejection, missing source error propagation, and readable byte formatting.

## Commands

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
```

## Additional cases to add

- [ ] Existing target overwrite behavior.
- [ ] Permission denied on source and target.
- [ ] Target parent directory missing.
- [ ] Large streamed input.
- [ ] Symlink alias between source and target.
- [ ] Disk-full or write interruption.
- [ ] Validate output using system gzip tooling where available.
- [ ] Failure cleanup after atomic output is implemented.

Report commands and results exactly. Do not invent coverage percentages.
