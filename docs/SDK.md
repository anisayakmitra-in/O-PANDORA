# Pandora SDK — Build Your First Gene

You'll have a working gene running in 5 minutes. A custom harness
takes about 20 more. No PhD required.

---

## 5-Minute Gene

### 1. Scaffold a gene

```bash
pandora new gene my-tool
```

This creates:

```
my-tool/
├── gene.toml       # id, name, kind, version, permissions, trust
└── src/
    └── lib.rs      # Your gene implementation
```

### 2. Write your logic

Open `my-tool/src/lib.rs`. The scaffold gives you a `Gene` impl with
an `execute` method. Put your code there:

```rust
fn execute(&self, input: &str) -> Result<String, PandoraError> {
    // Your logic goes here
    Ok(format!("Done: {input}"))
}
```

### 3. Run it locally

```bash
pandora run --local ./my-tool "hello world"
```

The `--local` flag loads your gene straight from the directory — no
publishing, no K-O-Palace, no install step. Great for iterating.

### 4. When you're ready, install it

```bash
pandora install ./my-tool
pandora gene enable my-tool
pandora run "use my-tool to do something"
```

---

## 20-Minute Harness

A harness packages genes, policies, and slash commands for a domain.
Think of it as a themed toolbox.

### 1. Scaffold

```bash
pandora new harness my-domain --kind domain
```

Kinds: `domain` (most common), `source` (needs approval), `meta`
(coordination mesh).

### 2. Add genes to the harness

Edit `harness.toml`:

```toml
owned_genes = ["my-tool", "another-gene"]
```

### 3. Add capabilities

```toml
capabilities = ["data-processing", "csv-export"]
```

These capabilities are how the Shadow Council routes tasks to your
harness. When someone runs `pandora run "export data"`, the council
looks for harnesses that advertise `csv-export` or `data-processing`.

### 4. Wire it in

```rust
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]
pub struct MyHarness { m: HarnessManifest }

impl MyHarness {
    pub fn new() -> Self {
        let manifest = HarnessManifestBuilder::default()
            .id("my-domain")
            .name("My Domain Harness")
            .kind(HarnessKind::Domain)
            .version("0.1.0")
            .author("you")
            .description("Handles data tasks")
            .capability("data-processing")
            .capability("csv-export")
            .build()
            .expect("valid manifest");
        Self { m: manifest }
    }
}

impl Harness for MyHarness {
    fn manifest(&self) -> &HarnessManifest { &self.m }
    fn initialize(&mut self) -> Result<(), PandoraError> { Ok(()) }
    fn shutdown(&mut self) -> Result<(), PandoraError> { Ok(()) }
    fn health(&self) -> Result<(), PandoraError> { Ok(()) }
}
```

### 5. Install and run

```bash
pandora harness install ./my-domain
pandora harness enable my-domain
pandora run "export this quarter's data"
```

---

## Gene Kinds

| Kind | Use for |
|------|--------|
| `Tool` | Shell commands, file ops, APIs |
| `Agent` | Multi-step reasoning, sub-agent spawning |
| `Workflow` | Orchestrated multi-gene pipelines |
| `MCP` | Model Context Protocol tools |
| `Skill` | Inline agent skills (markdown + triggers) |
| `Provider` | Custom LLM backends |

List them: `pandora new gene --list-kinds`

---

## Permissions

Every gene declares what it can access:

```toml
[permissions]
filesystem = "read"    # read, write, or deny
network = "outbound"   # none, outbound, full
shell = "deny"         # deny, allow
```

The runtime enforces these. A gene that tries to use the network
when its permission is `none` gets blocked.

---

## Trust Levels

```toml
[trust]
level = "medium"         # low, medium, high
require_signature = true
```

Trust levels affect:
- Whether the gene can call other genes
- Whether Parliament inspects its tool calls
- Whether it needs signature verification on install

---

## What's Next

- **Programmatic use**: See `examples/runtime-hello/` for using
  `PandoraRuntime` directly from Rust.
- **Provider config**: `examples/provider-config/` shows how to
  set up LLM connections.
- **Session inspection**: `examples/session-read/` loads and
  inspects session history.
- **Skills**: `examples/skills/` has a simple greeting skill.
- **Custom harnesses**: `examples/custom-harness-domain/` is a
  worked example of a domain harness.

---

## Getting Help

```bash
pandora help              # All commands
pandora help run          # Help for a specific command
pandora doctor            # System health + security check
pandora providers         # List configured LLM connections
pandora genes list        # See installed genes
```
