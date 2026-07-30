# Graph and knowledge model

Pandora already contains two graph layers. They solve different problems and should stay separate.

## Task graph

`pandora-types::workflow_engine` owns `ExecutionGraph` and `WorkflowStep`.

- Steps are work units with dependencies.
- `topological_sort()` produces an execution order.
- `ready_steps()` identifies work whose dependencies are complete.
- `parallel` marks steps that can be considered for fan-out.
- `domain_hint` and `provider_hint` carry routing hints without hardcoding a domain into the CLI.

`pandora-orchestrator` builds the graph and runs the agentic loop. A verifier or approval gate belongs in the graph as a step or policy decision, not as an untracked prompt instruction.

The task-graph rules we are adopting are narrow:

1. Create an edge only when one step depends on another.
2. Fan out only independent work.
3. Give verification its own step and context.
4. Merge parallel results through one owned step.
5. Place approval before an operation that is expensive or hard to undo.
6. Stop when the workflow's stop condition is satisfied.

## Knowledge graph

`pandora-types::knowledge_distillation` owns `KnowledgeGraph`, `KnowledgeNode`, `KnowledgeCluster`, and `KnowledgeEdge`.

- Nodes carry a summary, confidence, tags, and source sessions.
- Edges carry an explicit relation and weight.
- Clusters group related execution knowledge.
- `KnowledgeTier` separates short-lived, retained, and durable knowledge.
- `KnowledgeQuery` provides the current query contract.

`pandora-types::provenance` owns the execution-specific provenance graph. Provenance answers where a result came from; distilled knowledge answers what should be retained for later work. Do not merge those stores without an explicit migration decision.

## Knowledge workflow

The current runtime path is:

```text
execution events
    -> recorder and telemetry
    -> failure analysis / knowledge distillation
    -> knowledge nodes and edges
    -> later session context
```

The current implementation keeps these graphs in runtime memory and session/event storage. It does not yet provide a full Markdown knowledge editor, semantic search index, or graph visualization client.

## OpenKnowledge-inspired boundary

A future knowledge surface can treat Markdown and project documents as user-owned source material while using Pandora's knowledge contracts for extracted summaries, links, provenance, and confidence. The editor should remain a client of the runtime, not a second memory implementation.

The safe order is:

1. Persist and version the existing knowledge graph contracts.
2. Add provenance for every extracted node and edge.
3. Add deterministic quality checks and deletion/retention behavior.
4. Expose read/write operations through the authenticated API.
5. Add a desktop graph and document view after the CLI/runtime contracts stabilize.

## What is not added

No new graph abstraction is needed now. The repository already has task graphs, provenance graphs, and knowledge graphs. New traits should only be introduced when an implementation must cross a crate boundary and no existing contract covers that boundary.
