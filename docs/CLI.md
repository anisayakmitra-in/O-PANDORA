# Pandora CLI Reference

Pandora's CLI is the primary interface. Every command follows:
`pandora <command> [args]`.

## Global flags

| Flag | Effect |
|------|--------|
| `--help`, `-h` | Show usage |
| `--version`, `-V` | Show version + platform + commit |
| `--json` | Emit machine-readable output for supported commands |

## Command map

The installed binary exposes these command groups:

| Group | Commands |
|------|----------|
| Execution | `run`, `route`, `execute`, `resume`, `replay`, `trace`, `inspect`, `explain`, `timeline`, `status`, `stop`, `export`, `overnight` |
| Providers | `setup`, `providers`, `model`, `connections`, `connection`, `benchmark`, `profiles` |
| Components | `harnesses`, `genes`, `gene`, `harness`, `service`, `new` |
| Governance | `governance`, `deny`, `approve`, `reject`, `rsi` |
| Packages | `install`, `uninstall`, `update`, `list`, `info`, `search`, `publish`, `package`, `artifacts`, `keygen`, `sign`, `verify` |
| Runtime | `serve`, `remote`, `fleet`, `config`, `keychain`, `graph`, `lineage` |
| Marketplace | `login`, `featured`, `trending`, `newest` |
| Utilities | `doctor`, `architecture`, `sessions`, `session`, `completions`, `import`, `version` |

Run `pandora <command> --help` for the accepted arguments and actions for any command.
## First-run setup

Run the guided wizard. It asks for the provider, endpoint, model, and connection name. For cloud providers, it reads the API key with hidden terminal input and stores only a credential reference:

```text
pandora setup
```

For automation or containers, configure a connection without prompts:

```text
pandora setup --provider ollama --model llama3.2
pandora setup --provider openai --endpoint https://api.openai.com/v1 --model gpt-4o
PANDORA_PROVIDER_API_KEY=... pandora setup --provider openai --model gpt-4o
```

For a secret supplied by a password manager or CI pipe, avoid shell history and process listings:

```text
Get-Content .\openai-key.txt | pandora setup --provider openai --model gpt-4o --api-key-stdin
printf %s "$OPENAI_API_KEY" | pandora setup --provider openai --model gpt-4o --api-key-stdin
```

Use `--non-interactive` to fail instead of opening the wizard when required flags are missing. `--api-key-stdin` requires redirected input and cannot be combined with `--api-key`.

For a clean machine, the shortest supported flow is:

```text
pandora doctor
pandora setup
pandora run "inspect this project"
```

Use `pandora --json ...` in scripts. Commands return non-zero status on invalid input or failed execution; do not parse human-readable output. The JSON execution report includes the execution ID, provider, model, duration, workflow steps, telemetry spans, knowledge and ledger counts, replay ID, and success status.

`pandora run --model NAME ...` overrides the selected model for one task. Use a profile when the provider, policy, and execution settings should be reused.

## Shell completion

Generate a completion script from the installed CLI.

```bash
# Bash
pandora completions bash > ~/.local/share/bash-completion/completions/pandora
# Zsh
pandora completions zsh > ~/.zfunc/_pandora
# Fish
pandora completions fish > ~/.config/fish/completions/pandora.fish
```

On Windows PowerShell, run `pandora completions powershell` and add the output to `$PROFILE`. Elvish is available with `pandora completions elvish`.

## `pandora doctor`

Run local health checks before the first task. Human output is concise; `pandora --json doctor` returns `api_version`, `checks`, `security`, `dependencies`, and `sessions`. Each check includes `ok`, `check`, `message`, and `remediation`. Add `--strict` when automation should return exit code `1` if a required check fails. Optional tools remain diagnostic-only. Without `--strict`, doctor remains informational and returns success after producing diagnostics.

```text
pandora doctor
pandora --json doctor
pandora --json doctor --strict
```

Missing optional tools are reported as diagnostics. Install a tool only when the workflow that needs it requires it.
## Execution commands

### `pandora remote <action> [endpoint] [task]`

Use an authenticated remote runtime node. Set `PANDORA_API_TOKEN` before protected requests:

```text
pandora remote health http://127.0.0.1:9090
pandora remote info http://127.0.0.1:9090
export PANDORA_CREDENTIALS_KEY="<random-secret-kept-out-of-source-control>"  # Linux/headless fallback
pandora remote pair http://127.0.0.1:9090 "$PANDORA_PAIRING_CODE"
pandora remote add http://127.0.0.1:9090 my-node
pandora remote list
pandora remote revoke http://127.0.0.1:9090
pandora remote run http://127.0.0.1:9090 "inspect this project"
```
### `pandora rsi <list|show>`

Inspect GEPA proposals produced from failed sessions. RSI is review-only: Pandora does not replace a running service or mark a proposal applied until a verified DSR activation path exists.

```bash
pandora rsi list
pandora --json rsi list
pandora rsi show <proposal-id>
```

### `pandora deny <list|add|remove>`

Manage user-level shell deny patterns. Deny rules are stored in the Pandora configuration and are evaluated before tool execution. They take precedence over tool-level approval settings.

```bash
pandora deny list
pandora deny add "sudo *"
pandora deny add "rm -rf *"
pandora deny remove "sudo *"
```

Patterns use Pandora's existing glob matching (`*` matches any sequence). Keep rules narrow enough to avoid blocking unrelated work.

### `pandora export [session-id]`

Exports one session or the complete local session history. JSON is the default format; Markdown is available for readable reports. Use `--redact` to replace credential-like metadata fields, `--output=-` for stdout, or `--output=path` to write a file.

```bash
pandora export --format=json --redact --output=sessions.json
pandora export <session-id> --format=markdown --output=session.md
```

Exports do not delete or mutate the session store. Treat unredacted exports as sensitive because prompts, tool results, paths, and artifacts may contain user data.

### `pandora serve [address]`

Starts the local runtime API on `127.0.0.1:9090` by default. Pass an explicit address for a remote node; protected endpoints then require `PANDORA_API_TOKEN`. Clients should use the versioned `/api/v1` routes. The authenticated WebSocket stream is available at `/api/v1/ws`; send an `ExecuteRequest` JSON object and receive versioned `EventEnvelope` messages.
Authenticated clients can inspect durable transport records at `/api/v1/deliveries`. These records contain serialized runtime responses or events, so access should be limited to trusted operators.

### `pandora route <task>`

Preview capability routing without calling a provider. This shows the required capabilities, selected harness, optional gene, score, and rationale.

```text
pandora route "design an accessible settings screen"
pandora --json route "review this Rust parser for security issues"
```

### `pandora profiles [NAME]`

List profile names or inspect one profile, including its domain role and named model bindings. `--json` returns the same data without credentials.

```bash
pandora profiles
pandora profiles design
pandora --json profiles design
```
### `pandora run <task> [--profile NAME] [--model NAME] [--output text|json] [--quiet] [--stream]`

Execute a task through the pipeline. A profile supplies provider, strategy, evaluator, approval, retry, and sandbox defaults. Domain profiles may also name role-specific connections and models; `run` rejects bindings that reference missing connections. Pandora resolves the domain from the task and the registered manifests; the command does not require a domain-specific flag. Add `--stream` to print provider chunks as they arrive when the provider supports streaming. Tool-calling turns remain buffered. `--stream` cannot be combined with JSON output.

```
pandora run "build a REST API in Rust"
pandora run --profile strict "review this repository"
pandora run --output json "inspect this project"
```

### `pandora shell`

Start interactive operator shell.

```
pandora shell
# Within shell:
#   /run <task>     Execute a task
#   /sessions       List sessions
#   /replay <id>    Replay execution
#   /help           Show commands
#   /quit           Exit shell
```

### `pandora resume [id]`

Resume interrupted execution from its last checkpoint.

### `pandora replay <id>`

Replay a completed execution.

### `pandora trace <id>`

Show execution trace with timing.

### `pandora inspect <id>`

Inspect execution state: plan, decisions, outcomes.

### `pandora explain <id>`

Explain why decisions were made.

### `pandora timeline [id]`

Show chronological event timeline.

## Provider commands

### `pandora providers`

List providers with health status.

### `pandora model [NAME]`

Show the persisted default model and configured connection models, or set a new default. An explicit `pandora run --model NAME` still overrides it for one task.

```text
pandora model
pandora model deepseek-coder
pandora --json model
```

### `pandora connection add <name> <kind> <endpoint> [model]`

Add a provider connection. With `--api-key`, Pandora stores the key through `pandora-secrets` and writes only a credential reference to `connections.toml`.

```
pandora connection add local ollama http://localhost:11434
pandora connection add openai-api openai https://api.openai.com/v1
```

### `pandora connection test <name>`

Test a connection health.

### `pandora connection remove <name>`

Remove a connection.

## SDK commands

| Command | Creates |
|---------|---------|
| `pandora new gene <name>` | Gene plugin (gene.toml + src/lib.rs) |
| `pandora new harness <name>` | Domain harness |
| `pandora new package <name>` | Distributable package |
| `pandora new evaluator <name>` | Quality gate evaluator |
| `pandora new skill <name>` | AI skill directory |
| `pandora new policy <name>` | Governance policy |
| `pandora new workflow <name>` | Workflow definition |
| `pandora new provider <name>` | LLM provider plugin |

### `pandora approve [id]` and `pandora reject [id]`

List pending approvals when no ID is supplied. Pass an ID to record the decision for that exact tool request. Use `--json` for automation.

## Package commands

`install`, `uninstall`, `update`, `list`, `info`, `search`, `publish`

## Package signatures

Generate a keypair once, then sign the exact archive hash that will be published. Pandora stores the resulting public signature metadata under the local Pandora data directory; the secret key is never written by the signing command.

```bash
pandora keygen
PANDORA_SECRET_KEY=... PANDORA_ARCHIVE_SHA256=sha256:... pandora sign acme/browser 1.2.0
pandora verify acme-browser-1.2.0
```

Set `PANDORA_PUBLISHER` when the signature must identify a publisher other than `local`.

## Migrating legacy provider credentials

Older Pandora versions could write `api_key` into `connections.toml`. Move those keys into the configured secret source with:

```text
pandora keychain migrate
```

The command rewrites the connection file with credential references. It does not print key values. Back up the file before migration if you need a manual rollback.

## Uninstalling the CLI

Remove the installed binary without deleting sessions, credentials, or project data:

```bash
PANDORA_INSTALL_DIR="$HOME/.local/bin" bash scripts/uninstall-cli.sh
# Windows PowerShell:
$env:PANDORA_INSTALL_DIR="$HOME\\.pandora\\bin"; powershell -ExecutionPolicy Bypass -File scripts/uninstall-cli.ps1
```

## Utility commands

`doctor` - run system diagnostics
`harnesses` - list registered harnesses
`keygen` - generate an Ed25519 signing key
`sessions` - list recent sessions

## Related

- [SDK guide](SDK.md)
- [Configuration](CONFIGURATION.md)
- [Permissions](PERMISSIONS.md)


## CLI update and rollback

Use the platform helper to download, verify, health-check, and replace the CLI. A failed health check restores the `.previous` binary.

```text
PANDORA_VERSION=<published-version> bash scripts/update-cli.sh
powershell -ExecutionPolicy Bypass -File scripts/update-cli.ps1
```
