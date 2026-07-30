# Domain Agents

Pandora treats a domain agent as a domain harness running an agent profile. It is not a fourth harness kind and does not replace the Source or Meta layers.

## Ownership

```text
Source Harness  plans, governs, and approves
Meta Harness    coordinates, evaluates, and records
Domain Harness  performs work within one domain
Genes           provide reusable capabilities
Provider        supplies model execution
```

`HarnessKind::Domain` remains the canonical classification. A package can present itself as a design, coding, research, security, or frontend agent through its manifest capabilities and agent profile.

## Current behavior

- `ShadowCouncil` routes tasks to enabled harnesses by intent and capability.
- `ConnectionRegistry` stores multiple local, cloud, and enterprise provider connections.
- Execution profiles select provider and runtime defaults.
- The CLI and desktop use the same authenticated execution API.
- K-O-Palace is the package and registry service.

The current profile format selects one provider policy for an execution. It does not yet provide per-role provider bindings inside a domain harness.

## Target profile

The next compatible extension should allow a signed domain package to declare named roles without storing credentials:

```toml
[domain]
role = "design"

[models.planner]
connection = "primary-planner"
model = "controller-model"

[models.execution]
connection = "design-model"
model = "design-model"

[models.review]
connection = "review-model"
model = "review-model"
```

Connections refer to entries managed locally by Pandora. API keys remain in the operating-system credential store or the encrypted headless fallback. A package must never ship a key.

## Routing

A request should resolve in this order:

1. Explicit user or profile selection.
2. Domain capability match through `ShadowCouncil`.
3. Role-specific provider and model constraints.
4. Local policy, capability leases, budget, and approval requirements.
5. Provider health, fallback, and audit recording.

The planner may propose a domain route. It may not bypass capability checks, approvals, sandbox boundaries, or credential isolation.

## Evolution

GEPA may compare domain-agent outcomes and propose profile or prompt changes. RSI may prepare a bounded improvement proposal. DSR may produce a signed replacement package with a hash and rollback target. Activation remains an explicit governed operation; no domain agent may silently modify its own runtime, credentials, or governance rules.

## K-O-Palace packages

A registry package may advertise:

- domain and capabilities;
- supported provider protocols and model roles;
- required tools and permissions;
- evaluator and compatibility requirements;
- package hashes, signatures, and rollback metadata.

Pandora verifies these declarations before installation and applies local policy before execution.
