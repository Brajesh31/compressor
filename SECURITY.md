# 🔐 Security Policy

## Reporting

Do not publish exploit details or sensitive file contents in a public issue. Use GitHub's private vulnerability reporting feature if enabled, or contact the maintainer through a private GitHub channel.

## Scope

Compressor reads a caller-selected source file and writes a caller-selected output file. Security-relevant behavior includes path handling, file permissions, output truncation, disk exhaustion, malformed inputs if a decompression path is later added, and dependency integrity.

## Current limitations

- Target files are created or truncated.
- Same textual source and target paths are rejected, but symlink and hard-link equivalence is not fully checked.
- Output is written directly to the target, so an I/O failure may leave a partial file.
- No resource quota or maximum input size is configurable.
- The current tool does not expose a network listener or execute input content.

## Safe usage

- Run on files you are authorized to read and write.
- Choose an output path that does not contain valuable existing data.
- Avoid running with elevated privileges on untrusted paths.
- Review dependency updates and keep Cargo.lock committed.

Report vulnerabilities privately with Rust version, operating system, affected commit, safe reproduction steps, and expected/actual behavior. Remove personal information and sensitive file contents.
