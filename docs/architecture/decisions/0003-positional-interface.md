# ADR-0003: Two Positional File Paths

- **Status:** Observed current architecture

## Context

The current utility has one primary operation and few configuration options.

## Current choice

The CLI accepts exactly two positional arguments: source and target.

## Consequences

Simple usage and no argument-parser dependency. No built-in flags, output naming rules, verbosity controls, or JSON output mode.

## Revisit when

Compression levels, decompression, directory operations, or output policies require structured options.
