# Memory Specification

## What is this?

Pandora stores conversation context, learned facts, and execution knowledge in a hierarchical memory system. Three layers, each with different retention and purpose.

## When is it used?

During every execution. The Context Manager pulls relevant memories before each LLM call, and stores new knowledge after each step.

## Layers

| Layer | Retention | Scope | Example |
|-------|-----------|-------|---------|
| `Working` | Current session only | Full conversation | "User asked for a REST API" |
| `Episodic` | Recent sessions | Summaries | "Yesterday built a CLI tool" |
| `Semantic` | Permanent | Distilled facts | "User prefers Rust" |

## API

```rust
let mut mem = HierarchicalMemory::new();

// Store in working memory
mem.remember(MemoryLayer::Working, "user wants Rust", vec!["preference"]);

// Search across all layers
let results = mem.search_by_tags(&["preference"], None);

// Search specific layer only
let results = mem.search_by_tags(&["preference"], Some(MemoryLayer::Semantic));
```

## Context strategy

When the context window exceeds the token limit, a strategy applies:

| Strategy | What happens |
|----------|-------------|
| `TruncateOldest` | Drop oldest messages |
| `Summarize` | Replace old messages with a summary |
| `Priority` | Keep messages marked high-priority |

Default: `TruncateOldest`.

## Persistence

Memory is stored under `~/.pandora/sessions/`. Each session has its own directory. The `EventStore` writes events to `~/.pandora/checkpoints/`.

## How to extend

Add a new `MemoryLayer` variant. The `HierarchicalMemory` struct handles routing. No core changes needed.