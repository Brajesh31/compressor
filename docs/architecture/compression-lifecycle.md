# 🔄 Compression Lifecycle and Failure Paths

## Successful path

~~~mermaid
sequenceDiagram
  actor User
  participant CLI
  participant FS as Filesystem
  participant Buffer as BufReader
  participant Encoder as Gzip encoder
  User->>CLI: compressor source target.gz
  CLI->>CLI: Validate argument count and paths
  CLI->>FS: Open source and read metadata
  CLI->>FS: Create/truncate target
  CLI->>Buffer: Read source bytes
  Buffer->>Encoder: Stream bytes
  Encoder->>FS: Write compressed stream
  CLI->>Encoder: finish()
  Encoder-->>CLI: Finalized output handle
  CLI->>FS: Read output metadata
  CLI-->>User: Sizes, ratio, elapsed time
~~~

## Failure path

~~~mermaid
flowchart TD
  Start([Start]) --> Args{Exactly two paths?}
  Args -->|No| Usage[Print usage] --> Fail[Exit non-zero]
  Args -->|Yes| Same{Same path?}
  Same -->|Yes| Fail
  Same -->|No| Open[Open source]
  Open -->|Error| Fail
  Open -->|OK| Create[Create target]
  Create -->|Error| Fail
  Create -->|OK| Copy[Stream through Gzip encoder]
  Copy -->|I/O error| Partial[Target may be partial] --> Fail
  Copy -->|OK| Finish[Finalize encoder]
  Finish -->|Error| Partial
  Finish -->|OK| Report[Read metadata and report]
  Report -->|Error| Fail
  Report -->|OK| Done([Exit success])
~~~

## Correctness invariants

- Gzip output is not complete until the encoder is finalized.
- Output metadata is read after finalization.
- Source data is streamed rather than buffered entirely in memory.
- Errors propagate to the entry point and produce a non-zero process exit.
- The output path may be truncated before the full operation succeeds.
