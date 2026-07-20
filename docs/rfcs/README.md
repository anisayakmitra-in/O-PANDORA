# RFC Process — Pandora Architecture

## When to write an RFC

Any change to a **frozen API surface** requires an RFC. Frozen surfaces include:

- Execution Pipeline (stages, controller)
- Universal Registry (traits, entries)
- RuntimeNode (kinds, transports)
- Permission Manifest (structure, verdicts)
- Event Bus (kinds, protocol)
- Intent Router (capacity model)
- Hierarchical Memory (layers)
- Context Strategy (strategies)
- Lifecycle Hooks (events)
- Model Registry (entries)
- Policy Engine (interfaces)
- Workflow Lifecycle (states, middleware)
- SDK (scaffold templates)

Bug fixes and test additions do not require RFCs.

## RFC template

Each RFC should follow this structure:

```markdown
# RFC-XXXX: Title

**Status:** Draft | Review | Accepted | Rejected | Implemented
**Author:** Name
**Date:** YYYY-MM-DD

## Motivation

Why this change is needed. What problem does it solve?

## Design

The proposed design. Include data structures, interfaces, and how
it connects to existing subsystems.

## Impact on frozen surfaces

Which frozen APIs are affected? What migration is required?

## Capabilities

What new capabilities does this introduce? What well-known
capability strings should be added to `capability_registry::well_known`?

## Alternatives considered

What other approaches were evaluated? Why were they rejected?

## References

Related issues, prior art, external specifications.
```

## RFC lifecycle

1. **Draft:** Author writes the RFC
2. **Review:** Community discusses, author revises
3. **Accepted:** Approved for implementation
4. **Implemented:** Code merges with passing tests
5. **Rejected:** Closed with documented rationale

## Accepted RFCs

| Number | Title | Status |
|--------|-------|--------|
| 0001 | Capability System as Common Language | Implemented |

All RFCs live in `docs/rfcs/`.
