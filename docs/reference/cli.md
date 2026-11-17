# ⌨️ CLI Reference

## Synopsis

```text
compressor <source> <target.gz>
```

The current binary expects exactly two positional paths.

| Argument | Required | Description |
|---|---|---|
| source | Yes | Existing file to read |
| target.gz | Yes | Output path, created or truncated |

## Examples

```bash
compressor report.txt report.txt.gz
compressor ./data/archive.csv ./out/archive.csv.gz
cargo run --release -- input.bin output.bin.gz
```

## Exit behavior

- Exit code 0: compression and final reporting succeeded.
- Non-zero: invalid arguments or filesystem/encoding I/O failure.
- Error details are printed to stderr.

## Overwrite warning

The target file is created or truncated. Do not point it at important existing data. The current version rejects identical source and target path values but does not fully resolve all symlink/hard-link aliases.

## Output report

Source bytes, target bytes, output/input percentage, estimated space saved, and elapsed time. The ratio is unavailable for an empty source.
