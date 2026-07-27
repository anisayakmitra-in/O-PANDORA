# Screenshots

Real output captured from O-PANDORA v0.2.0.

## `pandora --version`

```
pandora 0.2.0
```

## `pandora doctor`

```
=== Pandora Doctor ===

--- Security ---
  API token set:       NO  (set PANDORA_API_TOKEN)
  Insecure mode:       NO
  Credentials stored:  NO
  Keychain available:  NO  (use file-based credentials)
  Dev mode:            NO

--- Environment ---
Ollama... OK
Ollama reachable... OK
Git... OK
Docker... OK
GitHub CLI... FAIL
cargo... OK
python3... OK
node... OK
rustc... OK

Sessions: 0 stored
Architecture: frozen
Runtime: 0.2.0
```

## `pandora new gene my-gene`

Creates a scaffolded gene with `gene.toml` and `src/lib.rs`.

```
$ pandora new gene my-gene
Created: my-gene/
  gene.toml   — manifest
  src/lib.rs  — Gene impl

$ tree my-gene/
my-gene/
├── gene.toml
└── src/
    └── lib.rs
```

## `pandora new harness my-domain --kind domain`

```
$ pandora new harness my-domain --kind domain
Created: my-domain/
  harness.toml  — manifest
  src/lib.rs    — Harness impl

Install with: pandora harness install my-domain
```
