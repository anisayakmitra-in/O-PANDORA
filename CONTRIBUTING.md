# Contributing

Pandora's frozen core (12 crates) is minimal. New capabilities should be built as genes, harnesses, or skills — not as new runtime crates.

## Before you contribute

1. Read `docs/ARCHITECTURE_DECISIONS.md` to understand why things are the way they are.
2. If you want to add a feature to the runtime, consider whether it can be a package instead.
3. If you want to modify the runtime, file an issue first.

## Package contributions

Build your gene/harness/skill as a standalone project with a `pandora.toml`. Publish to Palace. Don't modify the Pandora source.

## Runtime contributions

- `pandora-types`: Canonical types. Changes here affect all crates.
- `pandora-orchestrator`: The pipeline. Changes here affect all executions.
- `pandora-services`: Default service implementations.
- `pandora-shadow-council`: Harness/gene registry + lifecycle.
- `pandora`: CLI entry point.
- `pandora-palace`: Registry server.

## Style

- Behavior-preserving changes only.
- Every struct field must have a doc comment.
- Tests must pass (`cargo test --workspace`).
- No unwrap() in production paths — use `PandoraError` or `Result` propagation.

## License

MIT. All contributions are MIT-licensed.
