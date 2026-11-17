# 🔐 Security Policy

## Reporting

Do not publish exploit details or sensitive file contents in a public issue. Use GitHub's private vulnerability reporting feature if enabled, or contact the maintainer through a private GitHub channel.

## Scope

Compressor reads a caller-selected source file and writes a caller-selected output file. Security-relevant behavior includes path handling, file permissions, output truncation, disk exhaustion, malformed input if decompression is later added, and dependency integrity.

## Current limitations

- Target files are created or truncated.
- Same textual source and target paths are rejected, but symlink/hard-link equivalence is not fully checked.
- Output is written directly to the target, so I/O failure may leave a partial file.
- No resource quota or maximum input size is configurable.
- The tool does not expose a network listener or execute input content.

Use least privilege and only process files you are authorized to read and write. Report vulnerabilities privately with Rust version, OS, affected commit, and safe reproduction steps.
