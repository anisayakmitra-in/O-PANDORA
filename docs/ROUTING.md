# Routing

Pandora routes work through `ShadowCouncil`. The router selects an enabled harness from the request intent and required capabilities, then selects an optional compatible gene.

## Request order

1. Use explicit required capabilities when provided.
2. Derive capabilities from intent only when required capabilities are empty.
3. Reject requests that resolve only to `general`.
4. Filter disabled harnesses and an explicit `owner_harness` policy.
5. Score required-capability matches, preferred-capability matches, and name or ID matches.
6. Select the highest-scoring harness and record the match rationale.
7. Select the best enabled gene owned by that harness or explicitly assigned to it.

## Domain profiles

A profile may declare a domain role and named model bindings:

```toml
[domain]
role = "design"

[models.planner]
connection = "controller"
model = "controller-model"

[models.execution]
connection = "designer"
model = "design-model"

[models.review]
connection = "reviewer"
model = "review-model"
```

These fields identify connections by name. They never contain credentials. The `execution` binding now selects one named connection and model for a run. Planner and review bindings remain contracts until their pipeline stages are enabled.

## Next safe extension

Role-aware execution should be additive and explicit:

- resolve the domain route first;
- resolve a requested role such as `planner`, `execution`, or `review`;
- validate the named connection through the existing connection registry;
- apply capability, approval, sandbox, budget, and provider-health checks;
- emit the selected role, connection, and model in the execution record;
- fail closed when a required role binding is missing.

The router must not select credentials, bypass approvals, or allow a profile to widen its own permissions. GEPA, DSR, and RSI may propose bounded profile or package changes, but activation remains a governed operation with validation, signatures, hashes, and rollback metadata where applicable.