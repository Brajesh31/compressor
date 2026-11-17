# 🤝 Contributing

Thanks for improving Compressor. This is a compact Rust CLI project, so favor focused, tested changes over unnecessary frameworks.

## 🧭 Development setup

```bash
git clone https://github.com/Brajesh31/compressor.git
cd compressor
rustup update stable
cargo test
```

## 📐 Project conventions

- Target Rust 2021 and stable.
- Use `cargo fmt` formatting.
- Keep errors explicit. Avoid `unwrap()` and `expect()` in production paths.
- Prefer streaming I/O for large files.
- Preserve standard Gzip interoperability.
- Do not add a dependency when the standard library or existing crate is sufficient.
- Never add generated `target/` files.

## 🧪 Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## ✅ Pull request checklist

- [ ] Scope and rationale are clear.
- [ ] Tests cover success and failure paths.
- [ ] Formatting and Clippy pass.
- [ ] Gzip interoperability is preserved.
- [ ] Documentation and diagrams are updated.
- [ ] No credentials, generated binaries, or private data are included.
