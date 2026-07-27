# O-PANDORA CLI/TUI UX Audit

This is a user-experience review of the current `pandora` CLI and `pandora-tui` terminal dashboard. No runtime redesign is proposed — the architecture stays frozen. Findings are scored by severity: **P0** (confusing or broken), **P1** (polish), **P2** (nice-to-have).

---

## 1. Help & Discoverability

| Finding | Severity | Details |
|---------|----------|---------|
| **Manual help text out of sync with clap** | P0 | `usage()` at `main.rs:453` is hand-maintained. The `Commands` enum already carries clap docstrings, but the CLI prints the hand-rolled list. If a command is added to the enum but not to `usage()`, users never see it. The two already diverge: `usage()` omits `trending`, `newest`, `overnight`, `import`, `profiles`, `approve`, `reject`, `sign`, `verify`, `fleet`, `lineage`, `artifacts`, and several `--flags`. |
| **No `help` subcommand** | P0 | `pandora help <cmd>` does not exist. The shell's `/help` is a one-line summary, and `pandora --help` only prints the top-level list. Per-command help is unavailable. |
| **Subcommand hints are inconsistent** | P1 | Many commands print `Subcommand: list, inspect` on invalid input; others print `Usage: pandora gene <list|inspect> [id]`. Users get two different formats for the same mistake. |
| **Examples are hardcoded and static** | P1 | Examples in `usage()` are helpful but not generated from clap metadata. The connection example still uses `http://localhost:11434` without explaining model name syntax. |
| **No command aliases** | P2 | Common aliases (`ls`, `rm`, `up`, `doc`, `prov`) are absent. Power users would benefit. |
| **TUI tabs are decorative** | P1 | `pandora-tui` renders tabs but only one static view exists; arrow keys / number keys do not switch tabs. The tabs suggest navigation that does not work. |

**Recommended fix:**
- Delete `usage()` and let clap generate help. Use `clap::CommandFactory` to print a styled help page or keep a thin wrapper that reads the clap app and prints the same table.
- Add `pandora help <cmd>` by forwarding to `Cli::command().find_subcommand(cmd).unwrap().print_long_help()`.
- Standardize error hint format: `Usage: pandora <cmd> <subcmd> [args]` everywhere.

---

## 2. Errors

| Finding | Severity | Details |
|---------|----------|---------|
| **Error messages leak implementation terms** | P0 | `cmd_run` prints `Pipeline failed: {e}\nSuggestion: Is Ollama running?` for *any* error. If the failure is a missing provider, governance rejection, or a harness error, the user still sees the Ollama question. |
| **Empty success is misreported as model error** | P0 | `Ok(r) if !r.success` prints: "Pipeline returned empty — set PANDORA_DEFAULT_MODEL or add a connection...". A non-successful run is not necessarily an empty model response; the message blames the provider regardless of cause. |
| **Missing subcommand exits silently** | P1 | `cmd_gene`, `cmd_harness`, `cmd_service` use `return` instead of `process::exit(1)` on invalid subcommands. The shell returns `0`, so scripts and users cannot detect a bad invocation. |
| **Connection errors print raw internal strings** | P1 | `cmd_install` prints `Could not connect to K-O Palace: {e}` where `e` is a `reqwest`/`PandoraError`. Error messages should classify: network, 404, unauthorized, malformed package, etc. |
| **No structured error codes** | P1 | Every command exits `1` on failure. Admins and CI cannot distinguish config errors from runtime errors. |
| **TUI errors are invisible** | P2 | The TUI has no status bar for errors. If data loading fails, the dashboard still shows static zeros. |

**Recommended fix:**
- Introduce a small error-classification helper: match `PandoraError` variants and map each to a one-line human reason + next action. Example: `Authentication` → "Set PANDORA_TOKEN"; `NotFound` → "Check the package id"; `Execution` → "Run pandora doctor".
- Exit non-zero from invalid subcommands.
- Add a `PANDORA_EXIT_CODES` section to docs mapping 1/2/3/4 to error classes.

---

## 3. Doctor

| Finding | Severity | Details |
|---------|----------|---------|
| **Doctor does not check the actual runtime** | P0 | `cmd_doctor` runs `curl` and `git --version` but never validates the registry, Shadow Council, or any installed package. It reports `OK` if the Ollama HTTP endpoint returns any 2xx, even if the model is unusable. |
| **No `PANDORA_DEFAULT_MODEL` check** | P0 | `pandora run` fails with the suggestion to set `PANDORA_DEFAULT_MODEL`, but `doctor` does not report whether the model is configured. |
| **Doctor uses shell commands for everything** | P1 | `cmd_doctor` shells out to `curl`, `docker`, `gh`, `cargo`, `python3`, `node`, `rustc`. On Windows these commands may not be on PATH or may be named differently. |
| **Windows support is accidental** | P1 | `ck` chooses `cmd` on Windows, but `cargo --version`, `node --version`, etc., are not guaranteed in a `cmd` environment. The command is also not quoted safely. |
| **No output tells the user what to fix** | P1 | `FAIL` is printed without context or remediation link. A user seeing `FAIL` for Docker has no idea whether Docker is optional or required. |

**Recommended fix:**
- Add a `doctor` check that loads the active connection registry and tests the default provider with a small health probe (e.g., `/api/tags` or a lightweight prompt).
- Print a summary table: check, expected, actual, severity, fix link.
- Group checks into `Required`, `Recommended`, `Optional`.

---

## 4. Shell

| Finding | Severity | Details |
|---------|----------|---------|
| **Shell history is overwritten on every command** | P0 | `cmd_shell` reads history, then writes the entire vector back with `std::fs::write(&hp, history.join("\n"))` after each command. If two shells run concurrently, the second one overwrites the first. |
| **No input validation for command dispatch** | P1 | `cmd` is taken from `parts[0]`, then match arms repeat logic already in `dispatch()`. The shell is a parallel parser that can drift from the main CLI. |
| **No tab completion, hints, or line editing** | P1 | The shell uses plain `stdin.read_line`. Arrow keys, history recall, and Ctrl-C behavior are handled by the terminal, not the app, producing inconsistent UX. |
| **No prompt when no connection is configured** | P1 | The shell opens even if no providers exist. The first `/run` will fail with a provider error after the user has already typed a task. |
| **Help inside shell is one line** | P1 | `/help` prints a cramped one-line list. It does not explain `/goal`, `/agent`, or how to escape the shell. |
| **`/agent` spawns a detached process with no tracking** | P2 | `cmd_shell` spawns a background `pandora run` and prints "Subagent running in background". There is no PID, session id, or way to reattach. |
| **`/goal` loop is not really resumable** | P1 | The goal command increments `turns_used` and `total_tokens` in memory. On resume, the counter resets to 0, so the "resume" feature does not actually resume state. |
| **Shell prompt has no context** | P2 | `pandora>` is static. It could show the active provider, model, or goal name. |

**Recommended fix:**
- Use `rustyline` or `reedline` for the shell prompt: history, completion, hints, keybindings.
- Route all commands through the same `dispatch()` parser so `/run <task>` is exactly `pandora run <task>`.
- Append history atomically instead of overwriting.
- On shell start, print a banner showing provider status.

---

## 5. `pandora run`

| Finding | Severity | Details |
|---------|----------|---------|
| **Output is truncated to 2000 chars without warning** | P0 | `r.output.chars().take(2000).collect()` silently drops the rest. Users do not know truncation happened. |
| **No streaming feedback** | P1 | `pandora run` blocks until the runtime returns. For long tasks the user sees nothing. The streaming types exist but are not wired to the CLI. |
| **No --provider / --model / --budget flags** | P1 | The CLI forces the default provider. Users cannot run a quick task with a different model without editing config. |
| **No `--dry-run` or `--explain` mode** | P2 | Users cannot preview the plan before execution. |
| **Task args are joined by space, losing quotes** | P1 | `args[2..].join(" ")` collapses shell quoting. A task like `run "add 'foo bar'"` becomes ambiguous inside the shell. |

**Recommended fix:**
- Add `...` or a `[truncated: use --output-file]` marker when truncating.
- Print progress turns (e.g., "Turn 1/20...") as the runtime advances.
- Add `--provider`, `--model`, `--budget`, `--max-turns`, `--max-tokens`, and `--dry-run` flags to `Commands::Run`.

---

## 6. Providers

| Finding | Severity | Details |
|---------|----------|---------|
| **`providers` command looks healthy but does not test configured connections** | P0 | If `reg.connections` is empty, it prints a direct Ollama check. If connections exist, it prints `OK`/`OFF` based on `is_healthy()`, but `ConnectionRegistry::load()` does not refresh health. A stale connection is shown as `OK`. |
| **`connections` table misaligns on long names** | P1 | `println!` with fixed widths is brittle. A 30-character name overflows the 20-character column. |
| **`connection add` requires positional args with no interactive fallback** | P1 | Users must know the exact kind string (`ollama`, `openai-compatible`, etc.) and endpoint. Typos produce a usage error. |
| **No connection test after add** | P1 | `cmd_connection` stores the connection without verifying it. A typo in the endpoint is only discovered on the first `run`. |
| **No `--json` output for scripting** | P2 | The table is human-only; CI cannot parse provider status. |

**Recommended fix:**
- Always health-check the displayed connections and print `last_checked`.
- Use a small table formatter (e.g., `tabled` or `comfy-table`) instead of hand-rolled widths.
- Add `connection test <name>` and run it automatically after `add`.
- Add `--json` flag to provider/connection commands.

---

## 7. Harnesses

| Finding | Severity | Details |
|---------|----------|---------|
| **`harnesses` command reports hardcoded counts** | P0 | `cmd_harnesses` prints literal strings like `Domain: 7 ... Meta: 1 ... Source: 5`. It does not read the actual registry. If a new harness is added, the output is wrong. |
| **`harness <inspect>` is a stub** | P1 | It prints `Harness: {id}` without capabilities, genes, or health. |
| **No group/filter options** | P2 | Users cannot list only domain harnesses or search by capability. |

**Recommended fix:**
- Query `ShadowCouncil` summary and print real counts.
- `inspect` should print: id, kind, capabilities, genes registered, health, trust level.

---

## 8. Gene Creation

| Finding | Severity | Details |
|---------|----------|---------|
| **`pandora new gene` hardcodes `version = "0.2.0"`** | P0 | The scaffolded `gene.toml` uses `version = 0.2.0`. Since the user is creating a new gene, this should default to `0.1.0` or use a template parameter. |
| **`pandora new gene` uses `kind = Tool` but docs show `GeneKind` strings** | P1 | `Tool` is a `GeneKind` enum variant; a beginner may not know which variants are valid. The CLI should either accept a string choice or print the list. |
| **Scaffolded provider uses `Provider` trait without importing it** | P0 | `cmd_new` for `provider` writes `impl Provider for ...` without `use pandora_types::provider::Provider;`. The generated file will not compile. |
| **Scaffolded harness is empty and useless** | P1 | `cmd_new` `harness` writes only `pub struct FooHarness;`. It does not implement `Harness`, so it cannot be registered. |
| **No `--path` option for scaffolding location** | P2 | Everything is created in the current directory. |
| **No `pandora new skill` implementation detail shown** | P1 | `skill` delegates to `pandora_kuber::skill::scaffold`, but the user does not see what was created. |

**Recommended fix:**
- Use `env!("CARGO_PKG_VERSION")` or a default `0.1.0` for scaffolds.
- Generate compilable stubs: include the required imports and trait implementations.
- Add `--list-kinds` / `--list-templates` flags so users can discover valid values.

---

## 9. TUI Navigation

| Finding | Severity | Details |
|---------|----------|---------|
| **Tabs are non-interactive** | P0 | Six tabs are rendered (`Runtime`, `Genes`, `Harnesses`, `Providers`, `Plans`, `Palace`) but only the first is ever shown. Left/right arrow keys do nothing. |
| **No key legend other than `q quit · esc exit`** | P1 | Users do not know how to switch tabs or interact with data. |
| **Data is static placeholders** | P1 | `pandora-tui` reads `builtin::all()` and `ShadowCouncil::new()` but does not poll, load sessions, or connect to providers. The `Runtime: v1.0` is hardcoded. |
| **No focus or selection model** | P1 | There is no cursor, list selection, or detail view. |
| **Color palette is single-accent** | P2 | Deep pink (`Rgb(255, 20, 147)`) is used for logo, tabs, and status. Status colors (green/yellow/red) are missing. |
| **Terminal resize handling is unverified** | P2 | The layout uses percentages, but very small terminals will likely truncate text. |

**Recommended fix:**
- Either implement tab switching with number keys and arrow keys, or remove the tabs and label the view as a dashboard.
- Add a proper footer key legend.
- Use status colors: green for healthy, yellow for warnings, red for errors.
- Add a selected list widget and detail pane.

---

## 10. Key Bindings

| Finding | Severity | Details |
|---------|----------|---------|
| **TUI: `q` and `Esc` only** | P1 | No `1`-`6` for tabs, no `Tab`/`Shift-Tab`, no `Enter` to drill into a row. |
| **Shell: `/` commands are not documented in `--help`** | P1 | The slash commands only appear inside the shell. A user reading `--help` will not know the shell exists or how to use it. |
| **No `Ctrl-C` handler in shell** | P1 | `read_line` may receive EOF or a signal; the shell currently breaks silently. |
| **No readline-style editing** | P1 | No `Ctrl-A`, `Ctrl-E`, `Ctrl-R`, up/down history. |

**Recommended fix:**
- Adopt a readline library for the shell.
- Document the shell and its slash commands in `--help` and the manual.
- Add tab-switching keys to the TUI.

---

## 11. Output Formatting

| Finding | Severity | Details |
|---------|----------|---------|
| **Mixed `println!` and `eprintln!` with no output abstraction** | P1 | Every command formats its own strings. Some use `println!`, some use `eprintln!` for errors. There is no shared output formatter or `--json` mode. |
| **Tables are hand-formatted with brittle widths** | P1 | `providers`, `connections`, `featured`, `trending`, `newest`, and `palace` all use `println!` with manual spacing. |
| **ASCII art overflows on narrow terminals** | P2 | `PANDORA_ASCII` and `PANDORA_LOGO` are wide. On 80-column terminals they wrap awkwardly. |
| **No timestamps in long-running output** | P2 | `overnight` and `run` do not prefix log lines with time. |
| **Box-drawing in `cmd_palace_shell` is fragile** | P2 | The palace shell uses hand-drawn boxes. Resizing or narrow terminals break the layout. |

**Recommended fix:**
- Introduce a small `Output` helper: default table, `--json`, and `--no-color` support.
- Replace hand-rolled tables with `comfy-table` or `tabled`.
- Add a `--width` guard or use terminal width to decide whether to show the logo.

---

## 12. Colors

| Finding | Severity | Details |
|---------|----------|---------|
| **CLI has no color support** | P1 | The CLI prints only plain text. Errors, warnings, and success are not color-coded. |
| **TUI uses a single accent color everywhere** | P2 | Deep pink is used for logo, tabs, and status bar. Different semantic colors (green for OK, red for errors) would improve scannability. |
| **No `NO_COLOR` / `--no-color` handling** | P2 | Pipelines and logs cannot opt out of color. |

**Recommended fix:**
- Use `anstream` or `owo-colors` for CLI color with `NO_COLOR`/`--no-color` support.
- In TUI, reserve deep pink for brand/logo, and use green/yellow/red for status.

---

## 13. Progress Bars

| Finding | Severity | Details |
|---------|----------|---------|
| **No progress bars anywhere** | P1 | Long commands (`run`, `overnight`, `benchmark`, `install`, `publish`) produce no progress indication. The terminal appears frozen. |
| **Benchmark prints a static table** | P1 | `cmd_benchmark` shows results only after all probes finish. Users wait without feedback. |
| **Install gives no feedback during resolution** | P1 | `cmd_install` is silent while resolving dependencies. |

**Recommended fix:**
- Add `indicatif` spinners for `install`, `benchmark`, `search`, and `publish`.
- For `run`, print a spinner during the first provider call and then turn logs into streaming output.

---

## 14. Summary of Severity

| Severity | Count | Examples |
|----------|-------|----------|
| P0 | 13 | help out of sync, provider health not tested, run output truncated, harness counts hardcoded, TUI tabs non-interactive, TUI static data, scaffold version hardcoded, provider stub missing import, doctor missing runtime check, shell history overwrite, error message blames Ollama, empty success misreported, connection registry stale health |
| P1 | 24 | invalid subcommands exit 0, no readline, no per-cmd help, no `--json`, no `--provider`, no `--dry-run`, no timestamps, no color, no progress bars, table misalignment, Windows doctor fragility, etc. |
| P2 | 12 | aliases, no command shortcuts, no `--path`, no `--width`, no focus model, no `NO_COLOR` passthrough, etc. |

---

## 15. Recommended Immediate Actions (P0/P1)

1. **Replace `usage()` with clap-generated help** and add `pandora help <cmd>`.
2. **Classify errors by `PandoraError` variant** and print actionable next steps.
3. **Fix `pandora new provider` scaffold** to include the required import.
4. **Make `harnesses` read real registry counts** instead of hardcoded strings.
5. **Add health checks to `doctor`** for the default provider and configured model.
6. **Fix shell history** to append atomically and use a readline library.
7. **Truncate `run` output with a visible marker**.
8. **Refresh connection health before displaying** in `providers`.
9. **Implement TUI tab switching** or remove the tabs.
10. **Add `--json` and `--no-color` flags** to provider/connection and list commands.
11. **Add progress spinners** to `install`, `benchmark`, `run`, and `overnight`.
12. **Add color-coded output** to the CLI using `anstream` with `NO_COLOR` support.

---

*Audit completed. No runtime redesign proposed; all recommendations are presentation-layer and CLI-flow improvements.*
