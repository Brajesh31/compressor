# 🚦 CI and Release Automation

## Current workflows

- `.github/workflows/ci.yml`: formatting, Clippy, tests, and release build on pushes and pull requests.
- `.github/workflows/codeql.yml`: scheduled and event-driven Rust static analysis.
- `.github/dependabot.yml`: weekly Cargo and GitHub Actions dependency proposals.

## CI flow

~~~mermaid
flowchart TD
  A([Push / pull request]) --> B[Checkout]
  B --> C[Install stable Rust]
  C --> D[Cache Cargo]
  D --> F[Format check]
  D --> G[Clippy]
  D --> H[Unit tests]
  D --> I[Release build]
  F --> J{All jobs successful?}
  G --> J
  H --> J
  I --> J
  J -->|Yes| K([CI green])
  J -->|No| L([Review failure])
~~~

CI validates source quality and release-binary build. A release still requires manual review of behavior changes, supported platforms, Cargo.lock updates, and Gzip interoperability.
