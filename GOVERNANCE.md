# 🏛️ Repository Governance

## Scope

Compressor is a Rust command-line utility that streams one input file into a Gzip output using `flate2`.

## Principles

1. Keep the binary small and its behavior predictable.
2. Prefer streaming I/O over loading complete files into memory.
3. Propagate errors instead of panicking in normal user workflows.
4. Preserve standard Gzip compatibility.
5. Keep dependencies minimal and Cargo.lock committed.
6. Document actual behavior separately from roadmap proposals.

## Change requirements

- Changes to CLI behavior update the reference documentation and tests.
- Changes to output semantics describe overwrite, permissions, and partial-file behavior.
- Compression changes include round-trip and representative input tests.
- Dependencies require a reason and lockfile review.

## Releases

Release notes must identify the version, commit, validation commands, behavioral changes, and known limitations. A successful build does not prove benchmark performance or production suitability.
