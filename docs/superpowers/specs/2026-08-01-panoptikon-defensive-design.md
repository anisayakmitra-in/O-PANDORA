# Panoptikon Defensive Meta Harness

## Goal

Add Panoptikon as a built-in, installable Pandora meta harness for defensive review of coercive surveillance and population-control designs. It preserves the supplied names as taxonomy labels and performs no surveillance, profiling, targeting, manipulation, or enforcement.

## Boundary

Panoptikon accepts user-supplied architecture and policy descriptions. It returns defensive findings, evidence requests, impact statements, and countermeasure guidance. It does not collect external data, connect to sensors or accounts, score people, generate target lists, alter content, control infrastructure, or execute actions.

The code lives in `pandora-harnesses` because it is a built-in implementation. `pandora-types` stays unchanged. The registry must never execute code downloaded from K-O Palace.

## Runtime Shape

`PanoptikonMetaHarness` uses `HarnessKind::Meta`, ID `panoptikon-meta`, the defensive `argos-perpetua` capability, and three descriptive commands:

- `/panoptikon.scan`: identify coercive-system indicators in supplied material.
- `/panoptikon.map`: map indicators to privacy, due-process, safety, and governance controls.
- `/panoptikon.counter`: produce defensive mitigations for reviewed findings.

The harness owns 31 manifest-only `Gene` values. `ARGOS-PERPETUA` remains a named harness capability because the supplied taxonomy identifies it as the native sensory layer rather than a numbered module. Genes are discoverable, routable, and enableable like existing preloaded genes. Their execution path returns the existing “not implemented” outcome; they do not perform side effects.

## Preserved Gene Taxonomy

| ID | Defensive scope |
|---|---|
| `cassandra-inverted` | Audit predictive-risk and preemptive-decision systems. |
| `panoptes-score` | Audit discriminatory scoring and access consequences. |
| `ananke-bind` | Detect coercive dependency and exclusion mechanisms. |
| `echo-chamber` | Assess information-integrity and recommendation risks. |
| `mnemosyne-override` | Assess record-integrity and historical-data tampering risks. |
| `eris-seeder` | Detect coordinated division, manipulation, and influence risks. |
| `oracle-inversion` | Assess truthfulness, provenance, and contestability controls. |
| `iron-mirror` | Detect social-isolation and retaliation patterns. |
| `narcissus-trap` | Assess personalized persuasion and consent risks. |
| `sisyphus-engine` | Detect procedural friction and denial-of-service-by-process risks. |
| `atlas-weight` | Assess collective-liability and unfair-burden mechanisms. |
| `scylla-charybdis` | Detect coercive-choice and consent failures. |
| `shepherd` | Assess movement-control and assembly-rights risks. |
| `typhon-net` | Assess critical-infrastructure abuse and resilience controls. |
| `cerberus-gate` | Audit access-control fairness and appeal paths. |
| `medea-calculus` | Detect family and relationship-based coercion risks. |
| `hades-ledger` | Audit penalty records, due process, and correction rights. |
| `leviathan-protocol` | Assess mass-impact interventions and emergency safeguards. |
| `lethe-protocol` | Assess identity, deletion, and record-erasure abuse risks. |
| `prometheus-chain` | Detect innovation suppression and unfair restriction risks. |
| `thanatos-archive` | Assess irreversible-decision safeguards and human review. |
| `basilisk-watch` | Audit biometric surveillance and consent boundaries. |
| `gorgon-array` | Assess sensitive-signal collection and minimization controls. |
| `nemesis-trace` | Audit network-analysis privacy and association risks. |
| `proteus-mask` | Detect concealment, audit-evasion, and accountability gaps. |
| `hydra-cell` | Assess distributed-system accountability and shutdown controls. |
| `kronos-keeper` | Audit temporal suppression and information-access risks. |
| `charon-bridge` | Assess migration, border, and eligibility decision safeguards. |
| `sphinx-gate` | Audit resource-access eligibility and appeal mechanisms. |
| `ouroboros-loop` | Detect self-reinforcing policy and feedback-loop risks. |
| `moloch` | Assess irreversible high-impact decision controls. |

## Distribution

Pandora ships the compiled implementation and recognizes only `panoptikon-meta`. A K-O Palace listing represents package identity, version, metadata, and installation state. Installing that listing enables the shipped implementation when the Pandora release contains it. It cannot introduce executable code.

The listing must declare the package as metadata-only, use a stable version, carry the same ID and owned-gene list, and include no artifact URL until K-O Palace enforces transactional artifact verification.

## Files and Tests

- Add `legacy/crates/pandora-harnesses/src/panoptikon.rs` for the meta harness and manifest-only genes.
- Update `legacy/crates/pandora-harnesses/src/lib.rs` to export, seed, recognize, and enable `panoptikon-meta`.
- Add focused tests for meta classification, command list, the exact 31 unique gene IDs, defensive-only capabilities, registration, and routing.
- Add a K-O Palace package metadata fixture only after its publication API can persist a verified package record. Do not claim a remote upload before that exists.

## Error Handling and Verification

Unknown package IDs remain inert. Duplicate IDs preserve the existing installed user choice. A manifest validation failure must prevent registration without partially enabling genes. Tests run with a unique `PANDORA_HOME` and serialized environment changes until the broader test-isolation defect is repaired.

Required checks:

```text
cargo fmt --all -- --check
cargo check --workspace --locked
cargo clippy -p pandora-harnesses --all-targets -- -D warnings
cargo test -p pandora-harnesses --lib -- --test-threads=1
cargo test --workspace --lib --tests -- --test-threads=1
```
