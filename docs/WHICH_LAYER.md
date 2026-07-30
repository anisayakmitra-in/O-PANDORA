# Choosing a Pandora layer

Start with the smallest unit that owns the behavior.

| Need | Build | Where it lives |
|---|---|---|
| Permanent runtime function | Service | `pandora-types` contract, `pandora-services` implementation |
| Extend one service | Source Harness | `pandora-harnesses` |
| Coordinate services or harnesses | Meta Harness | `pandora-harnesses` or an external package |
| Package one domain's behavior | Domain Harness | `pandora-harnesses` or an external package |
| Add one executable operation | Gene | `pandora-genes` or an external package |
| Reuse instructions and genes | Skill | K-O-Palace package |
| Order dependent steps | Workflow | K-O-Palace package or runtime configuration |
| Judge an outcome | Evaluator | K-O-Palace package or runtime configuration |
| Add a model backend | Provider/Connection | provider adapter and local connection registry |
| Add a user command | Slash command | harness or gene manifest |
| Distribute a component | Package | K-O-Palace |
| Add a client view or command | Adapter | CLI, API, or TUI crate |

## Tests for placement

- If the behavior must exist in every Pandora runtime, define a service contract.
- If it extends one service, use a Source Harness.
- If it coordinates multiple components, use a Meta Harness.
- If it makes one domain easier to use, use a Domain Harness.
- If it performs one operation, use a Gene.
- If it combines existing pieces for reuse, use a Skill or Workflow.
- If it is only transport or presentation, keep it in a client adapter.

Keep credentials, provider keys, and user data out of packages. Let manifests declare capabilities and permissions; let local policy decide whether they run.
