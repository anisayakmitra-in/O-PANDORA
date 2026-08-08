# Evolution Architecture

Pandora supports controlled improvement without making the execution kernel self-modifying. Evolution is a package-level capability, not a runtime privilege.

## GEPA

GEPA is the proposal loop:

```text
completed session
      |
      v
observe failures and outcomes
      |
      v
create a versioned proposal
      |
      v
review, test, and approve
```

`pandora_orchestrator::engines::mutation::MutationEngine` implements the current GEPA strategy. It reads session history and writes mutation proposals. It does not apply them. Proposals carry their target, rationale, evidence, confidence, proposed package or plan change, and provenance.

## RSI orchestration

RSI is the bounded loop that connects GEPA to DSR:

1. observe completed sessions;
2. generate a GEPA mutation candidate;
3. hold the candidate at `AwaitingApproval`;
4. verify the replacement package and compatibility;
5. prepare a DSR request with an approval and rollback target;
6. activate only between executions; and
7. record the result for the next observation cycle.

`pandora_orchestrator::engines::evolution::EvolutionEngine` creates the initial `AwaitingApproval` proposal in the RSI lifecycle. `pandora_orchestrator::engines::replacement::ReplacementEngine` validates the metadata required for a DSR request. Neither engine approves, installs, or activates a replacement.

## DSR

DSR is the replacement loop for an approved implementation:

```text
approved proposal
      |
      v
resolve package and compatibility
      |
      v
verify hash, signature, permissions, and tests
      |
      v
stage replacement between executions
      |
      v
activate, audit, and retain rollback
```

DSR must operate through the package and registry boundaries. It must not replace a service in the middle of a session, bypass the Shadow Council, or alter a running execution graph. The replacement record should include the old and new implementation IDs, versions, hashes, approval, compatibility result, activation time, and rollback target.

## Ownership

- `pandora-types` owns serializable proposal, approval, replacement, and audit contracts when those contracts are implemented.
- `pandora-orchestrator` may observe executions and request a proposal; it does not own package installation or activation.
- `pandora-shadow-council` remains the runtime registry and capability router.
- `pandora-ko-palace` owns package verification, installation, version selection, and rollback integration.
- K-O-Palace is the registry service.

## Safety gates

Evolution is disabled for an active execution. A proposal is data until an authorized approval is recorded. A replacement is activated only after compatibility and policy checks pass, and every activation must be reversible. If any gate fails, Pandora keeps the current implementation and records the failure.

This document describes the target boundary. It does not claim that production DSR activation, signed evolution packages, or automatic rollback are implemented today.
