# ADR-0002: Streaming Gzip via flate2

- **Status:** Observed current architecture

## Context

Files may be larger than available memory.

## Current choice

BufReader feeds bytes to flate2's Gzip encoder via std::io::copy.

## Consequences

The entire source is not loaded into a byte vector. Memory is bounded by I/O buffers and encoder internals. The Gzip stream must be finalized before output size is measured. Compression level uses the library default.

## Revisit when

Benchmarks justify another buffer strategy, compression level, or parallel pipeline.
