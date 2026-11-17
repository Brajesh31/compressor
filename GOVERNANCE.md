# 🏛️ Repository Governance

## Scope

Compressor is a Rust CLI utility that streams one input file into Gzip output using flate2.

## Principles

1. Keep the binary small and behavior predictable.
2. Prefer streaming I/O over loading complete files into memory.
3. Propagate errors rather than panic during normal use.
4. Preserve Gzip compatibility.
5. Keep dependencies minimal and Cargo.lock committed.
6. Separate current behavior from roadmap proposals.

## Change requirements

CLI changes update reference docs and tests. Output changes document overwrite, permissions, and partial-file behavior. Compression changes include round-trip tests. Dependencies require a reason and lockfile review.

## Releases

Release notes identify the version, commit, validation commands, behavior changes, and known limitations. A successful build does not establish benchmark performance or production suitability.
