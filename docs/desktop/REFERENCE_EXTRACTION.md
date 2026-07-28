# Reference Extraction Report — Pandora Desktop

## OpenSail (TesslateAI)

| Pattern | Source | Purpose | Adopt/Adapt/Reject | Pandora Equivalent |
|---------|--------|---------|-------------------|-------------------|
| Tauri 2.0 desktop shell | desktop/src-tauri/ | Native cross-platform app wrapper | **ADOPT** | pandora-desktop/src-tauri/ |
| Sidecar orchestrator binary | binaries/tesslate-studio-orchestrator | Separate process for agent runtime | **REJECT** — Pandora already has PandoraRuntime as library | Link pandora-orchestrator directly |
| Workspace model (editor + terminal + Git + previews) | app/src/ | Integrated coding workspace | **ADAPT** — Pandora adds governance layer | File tree + Monaco editor + PTY terminal + Git panel |
| Architecture node graph | app/src/ | Visual runtime topology | **ADAPT** — make Pandora-native | Parliament→Shadow Council→Harness→Gene graph |
| Approval UI | app/src/ | Human-in-the-loop for agent actions | **ADOPT** — essential for Pandora governance | Approval card with risk, permissions, [y/n] |
| Agent fleet management | app/src/ | Multi-agent orchestration | **ADAPT** — Pandora has Fleet | Fleet panel showing RuntimeNodes |
| Marketplace (Palace equivalent) | app/src/ | Package discovery and install | **ADAPT** — Pandora has Palace/KUBER | Palace marketplace with genes, harnesses, skills |
| Project lifecycle | app/src/ | Recent projects, resume sessions | **ADOPT** | Project selector with Git status, last session |
| Runtime selection | app/src/ | Model/provider switching | **ADOPT** | Model selector using Pandora provider registry |
| Background jobs | app/src/ | Long-running task management | **ADAPT** — Pandora has scheduler | Scheduler panel with cron jobs |
| Local-first desktop/server | desktop/src-tauri/ | Works offline, syncs when connected | **ADOPT** | Local PandoraRuntime, Palace optional |
| Streaming responses | app/src/ | Real-time agent output | **ADOPT** | Event-based streaming through Tauri events |
| Secret storage | Stronghold plugin | Encrypted secrets | **ADAPT** — prefer OS keychain | Tauri stronghold or OS credential manager |

## Flock (Onelevenvy)

| Pattern | Source | Purpose | Adopt/Adapt/Reject | Pandora Equivalent |
|---------|--------|---------|-------------------|-------------------|
| Rust workspace core | crates/flock-core | Shared types and logic | **REJECT** — Pandora already has pandora-types | pandora-types crate |
| Tauri integration | flock-ui/src-tauri/ | Desktop app shell | **ADOPT** | pandora-desktop/src-tauri/ |
| React application shell | flock-ui/src/ | Frontend with Vite | **ADOPT** | pandora-desktop/src/ |
| Multi-agent representation | flock-agent/ | Agent state visualization | **ADAPT** — Pandora's agentic loop is different | Parliament + Shadow Council routing visualization |
| Graph/state patterns | flock-core/ | State management | **REJECT** — Pandora has its own state | Shadow Council + registries |
| Event streaming | Tauri events | Real-time updates | **ADOPT** | Tauri events for execution streaming |
| Process management | flock-ui/ | Desktop process lifecycle | **ADOPT** | PandoraRuntime lifecycle in Tauri |
| Multi-window support | Tauri windows | Multiple project windows | **ADOPT** (future) | One project per window |

## Palot (ItsWendell)

| Pattern | Source | Purpose | Adopt/Adapt/Reject | Pandora Equivalent |
|---------|--------|---------|-------------------|-------------------|
| Electron desktop shell | apps/desktop/ | Cross-platform app | **REJECT** — Tauri is lighter, Rust-native | Tauri 2.0 instead |
| Project switching | apps/desktop/src/ | Multiple projects | **ADOPT** | Project selector with recent list |
| Session management | apps/desktop/src/ | Session CRUD, resume | **ADOPT** | Session sidebar with today/yesterday |
| Streaming responses | apps/desktop/src/ | Real-time agent output | **ADOPT** | Token-level streaming through events |
| Tool call rendering | apps/desktop/src/ | Expandable tool output | **ADOPT** | Collapsible gene/tool execution blocks |
| Diff viewer | apps/desktop/src/ | Side-by-side diffs | **ADOPT** | Diff panel with accept/reject |
| File preview | apps/desktop/src/ | Inline file viewing | **ADOPT** | Monaco editor integration |
| Terminal output | apps/desktop/src/ | Integrated terminal | **ADOPT** | PTY terminal tabs |
| Model/provider controls | apps/desktop/src/ | Model switching | **ADOPT** | Model dropdown with health status |
| Multiple concurrent sessions | apps/desktop/src/ | Parallel agent sessions | **ADAPT** (future) | Fleet workers |
| Scheduling | apps/desktop/src/ | Cron-like task scheduling | **ADOPT** | Pandora cron integration |
| Server lifecycle | apps/desktop/src/ | Backend process management | **ADAPT** — PandoraRuntime is library, not server | Direct library API, no shell-out |
| Migration/import UX | apps/desktop/src/ | Import from other agents | **ADOPT** | pandora import hermes/claude-code/opencode |
| Conversation UX | apps/desktop/src/ | Chat interface patterns | **ADAPT** — add governance cards | Chat with approval cards, gene execution blocks |

## License Implications

- OpenSail: MIT license — patterns can be adapted with attribution
- Flock: Check license in repo
- Palot: Check license in repo
- All three: No code copied directly. Patterns extracted as architectural inspiration.

## Key Design Decisions

1. **Tauri over Electron**: Pandora is Rust. Tauri keeps Rust-native performance and avoids Chromium bloat.
2. **Direct library linking over sidecar**: PandoraRuntime is linked as a library, not spawned as a process.
3. **One runtime, multiple surfaces**: Desktop, CLI, TUI, Web all share the same PandoraRuntime.
4. **Governance-first UI**: Approval cards and Parliament visibility distinguish Pandora from other agents.
5. **Dynamic registries**: All Harness, Gene, Provider lists rendered from runtime data, never hardcoded.
