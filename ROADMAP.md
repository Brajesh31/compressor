# 🗺️ Roadmap

Roadmap items are ideas, not shipped capabilities.

## P0 · Reliability

- [ ] Atomic output writes using temporary file and rename.
- [ ] Detect source/target equivalence across canonical paths and filesystem identity where supported.
- [ ] Define output permissions and overwrite semantics.
- [ ] Add disk-full and permission-denied integration tests.

## P1 · CLI usability

- [ ] Configurable Gzip compression level.
- [ ] Optional output naming.
- [ ] Quiet and machine-readable JSON output modes.
- [ ] Structured argument parser when options justify it.

## P2 · Formats and workflows

- [ ] Separate decompression subcommand.
- [ ] Directory traversal and archive packaging with explicit archive semantics.
- [ ] Reproducible benchmarks across file types and compression levels.
- [ ] Optional parallel compression after profiling.

## Non-goals until designed

No image optimizer, cloud service, GUI, password encryption, ZIP/TAR support, or network API is currently included.
