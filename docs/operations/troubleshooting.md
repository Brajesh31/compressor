# 🧰 Troubleshooting

| Symptom | Likely cause | Action |
|---|---|---|
| Usage text appears | Missing or extra arguments | Provide exactly source and target |
| Permission denied | Cannot read source or create target | Check permissions |
| No such file or directory | Wrong source or target parent path | Verify paths and create parent directories |
| Target is larger | Input is small or already compressed | Normal for Gzip |
| Output incomplete after error | Write failed before encoder finalized | Remove partial target and retry after resolving |
| Binary not found | Not built or PATH missing | Run cargo build --release |

Include OS, architecture, Rust version, command shape, error, exit status, and approximate input size. Never attach confidential file contents.
