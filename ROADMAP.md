# 🗺️ Roadmap

Roadmap items are ideas, not shipped capabilities.

## P0 · Reliability

- [ ] Atomic output writes via temporary file and rename.
- [ ] Detect source/target equivalence across canonical paths and filesystem identity where supported.
- [ ] Preserve output permissions intentionally and document overwrite semantics.
- [ ] Add disk-full and permission-denied integration tests.

## P1 · CLI usability

- [ ] Add a proper argument parser only when options justify the dependency.
- [ ] Configurable Gzip compression level.
- [ ] Optional output naming and keep/remove source switches.
- [ ] Quiet and machine-readable JSON output modes.

## P2 · Formats and workflows

- [ ] Separate decompression subcommand.
- [ ] Directory traversal and packaging, with explicit archive semantics.
- [ ] Benchmark harness for representative file classes.
- [ ] Optional parallel compression after profiling.

## Non-goals until explicitly designed

No image optimizer, cloud service, GUI, password encryption, ZIP/TAR support, or network API is currently included.
