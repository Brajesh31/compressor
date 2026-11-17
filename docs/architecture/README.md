# 🏛️ Architecture Overview

## 🎯 Current scope

A single Rust binary accepts two filesystem paths and writes a Gzip-compressed output using `flate2`. It does not currently expose a service API or process directories recursively.

## 🧱 Logical architecture

~~~mermaid
flowchart TB
  CLI[Command line arguments] --> Main[Application entry point]
  Main --> Validate[Argument and path checks]
  Validate --> Reader[File + BufReader]
  Reader --> Encoder[flate2 GzEncoder]
  Encoder --> Writer[Output File]
  Writer --> Report[Metadata and elapsed time]
  Report --> Console[Terminal report]
~~~

## 📦 Dependency architecture

~~~mermaid
flowchart LR
  App[compressor binary] --> Std[Rust standard library]
  App --> Flate[flate2]
  Flate --> Miniz[miniz_oxide]
  Flate --> CRC[crc32fast]
  CRC --> Cfg[cfg-if]
  Miniz --> Adler[adler2]
~~~

The transitive graph is based on the checked-in Cargo.lock. Cargo may resolve newer compatible versions if the lockfile changes.

## 🛡️ Runtime boundaries

| Boundary | Current behavior | Concern |
|---|---|---|
| CLI | Reads two positional arguments | Clear validation and non-zero error exit |
| Source | Opens caller-selected file | Permission and path errors |
| Buffer | Uses BufReader | Bounded buffered read path |
| Encoder | Gzip default compression level | CPU vs output-size trade-off |
| Target | Creates/truncates caller-selected path | Existing output may be overwritten |
| Report | Prints file sizes and duration | Metrics are informational, not benchmark results |

## ❌ Explicit non-features

No image resizing, JPEG/PNG/WebP optimization, ZIP/TAR packaging, directory traversal, encryption, HTTP API, GUI, cloud storage, or decompression subcommand is currently implemented.
