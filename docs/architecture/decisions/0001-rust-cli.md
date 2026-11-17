# ADR-0001: Rust CLI with Minimal Dependencies

- **Status:** Observed current architecture

## Context

The repository is a small file compression command-line utility.

## Current choice

Rust 2021 binary, standard library for arguments and file I/O, flate2 for Gzip compression.

## Consequences

Small runtime surface and simple distribution. Cargo.lock supports reproducible dependency resolution. A GUI, server, or multi-command architecture is not justified by current scope.

## Revisit when

The project expands into additional formats, subcommands, or reusable library APIs.
