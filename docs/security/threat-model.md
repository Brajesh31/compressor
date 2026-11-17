# 🔐 Threat Model

## Assets

Source files; existing target files; disk capacity; integrity of compressed output; Rust dependencies and build artifacts.

## Trust boundaries

~~~mermaid
flowchart LR
  User[User-controlled arguments] --> Process[Compressor process]
  Process --> Source[(Source filesystem)]
  Process --> Encoder[Gzip encoder]
  Encoder --> Target[(Target filesystem)]
  Process --> Stdout[Terminal report]
  Process --> Stderr[Error output]
~~~

## Threat register

| Threat | Impact | Mitigation/current state |
|---|---|---|
| Wrong target path | Unintended overwrite | Usage documents target truncation |
| Same source and target | Self-overwrite | Equal path values rejected; symlink/hard-link aliases remain a limitation |
| Disk exhaustion | Partial or failed output | Errors propagate; quotas and atomic output are not implemented |
| Permission/path failure | Operation cannot complete | Report error and exit non-zero |
| Dependency compromise | Build/runtime risk | Cargo.lock, dependency review, CodeQL, update automation |
| Sensitive file selection | Data disclosure via chosen output path | Caller controls path; tool has no network transmission |

The application does not currently run as a network service or execute file contents. Filesystem safety risks still apply. Use least privilege and process only authorized files.
