# 📦 Gzip Format Notes

## What the program writes

The output is a Gzip member produced by `flate2::write::GzEncoder`. Gzip is a stream/container format commonly represented with the `.gz` extension. It is distinct from ZIP and TAR archive formats.

## Conceptual structure

~~~mermaid
flowchart LR
  Header[Gzip header] --> Deflate[DEFLATE compressed payload]
  Deflate --> Trailer[CRC32 and original size trailer]
~~~

## Interoperability

A standard Gzip decoder should restore the original byte sequence. Unit tests validate this by decompressing generated output and comparing bytes.

## Compression characteristics

- Compression is lossless.
- Already-compressed or high-entropy data may not shrink.
- Very small inputs can grow because container metadata adds overhead.
- Compression level affects CPU usage and output size; the current CLI uses the library default.
- Gzip is not encryption and does not hide sensitive information.
