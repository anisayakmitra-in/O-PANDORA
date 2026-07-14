# Recovery Document — `session.rs`

## Original path
`crates/pandora-types/src/session.rs`

## Original purpose
Defines the Session model (execution record) and SessionStore (in-memory
cache with JSON persistence). Every execution produces a Session that
ties together prompt, workflow, timeline, telemetry, artifacts, ledger,
and metadata.

## Public API
- `SessionStatus` enum — Pending, Running, Completed, Failed(String)
- `Session` struct — 13 fields (id, label, prompt, created_at, completed_at,
  status, workflow, timeline, ledger, artifacts, metadata, replay_id)
- `Session::new(id, prompt)` — constructor
- `Session::duration()` — elapsed time as Option<Duration>
- `Session::add_frame(frame)` — append to timeline
- `Session::add_artifact(path)` — append to artifacts
- `SessionStore` struct — in-memory HashMap<String, Session>
- `SessionStore::new()` — constructor (loads from disk silently)
- `SessionStore::save()` — persist all sessions as JSON files
- `SessionStore::load()` — load all sessions from disk
- `SessionStore::create(id, prompt)` — create or get existing
- `SessionStore::get(id)` — get by id
- `SessionStore::get_mut(id)` — mutable get by id
- `SessionStore::all()` — all sessions sorted by created_at desc
- `SessionStore::recent(n)` — last n sessions
- `SessionStore::by_status(status)` — filter by status
- `SessionStore::search(query)` — search by prompt or id
- `SessionStore::remove(id)` — remove from store and disk
- `SessionStore::count()` — total sessions
- `SessionStore::replay(id)` — create replay session from original

## Exported symbols
- `SessionStatus` (enum)
- `Session` (struct)
- `SessionStore` (struct)

## Dependency relationships
- Uses `crate::recorder::ExecutionFrame` (timeline)
- Uses `crate::PandoraError` (remove, replay error returns)
- Uses `serde` (serialize/deserialize derive)
- Uses `std::collections::HashMap`, `std::time::SystemTime`
- Used by: orchestrator (sessions.create, sessions.save), CLI (sessions,
  session, replay, delete-session), TUI

## Key algorithms
- `save()` — atomic write via tempfile + rename, writes index.json
- `load()` — first tries index.json, falls back to directory scan
- `create()` — checks existence, inserts new, persists immediately
- `replay()` — clones original prompt with [REPLAY] prefix

## Invariants
- Sessions are persisted after every create/remove
- Index file is maintained for ordered loading
- Atomic writes prevent partial file corruption
- `session_dir()` is always PANDORA_HOME/sessions/ or ~/.pandora/sessions/

## Restoration instructions
Replace with original. Session has 13 fields. SessionStore has 12 methods.
Standard HashMap-based store with JSON file persistence via serde.
