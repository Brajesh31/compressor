# 🤝 Contributing

Thanks for improving Compressor. This is a compact Rust CLI project, so favor focused, tested changes over unnecessary frameworks.

## Setup

```bash
git clone https://github.com/Brajesh31/compressor.git
cd compressor
rustup update stable
cargo test
```

## Conventions

- Target Rust 2021 and stable.
- Use cargo fmt formatting.
- Propagate errors; avoid unwrap/expect in production paths.
- Prefer streaming I/O for large files.
- Preserve standard Gzip interoperability.
- Keep dependencies minimal.
- Never commit generated target/ files.

## Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Pull request checklist

- [ ] Scope is clear.
- [ ] Tests cover success and failure paths.
- [ ] Formatting and Clippy pass.
- [ ] Gzip interoperability is preserved.
- [ ] Documentation and diagrams are updated.
- [ ] No credentials, generated binaries, or private data are included.
