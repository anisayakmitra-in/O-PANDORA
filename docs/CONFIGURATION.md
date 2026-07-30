# Configuration

O-PANDORA reads its defaults from `~/.pandora/config.toml`. You can override most settings with environment variables. The shortest setup path is documented in [CLI](CLI.md).

## Example

```toml
[defaults]
provider = "ollama"
model = "llama3.2"
sandbox = "none"
max_tokens = 4096
max_retries = 3

[connections.ollama]
kind = "ollama"
endpoint = "http://localhost:11434"
```

## Environment variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `PANDORA_HOME` | `~/.pandora` | Root data directory |
| `PANDORA_DEFAULT_MODEL` | (first available) | Default model name |
| `PANDORA_DEFAULT_PROVIDER` | (first available) | Default provider name |
| `PANDORA_MAX_TOKENS` | `4096` | Max tokens per execution |
| `PANDORA_MAX_RETRIES` | `3` | Max retries per step |
| `PANDORA_MAX_ATTEMPTS` | `3` | Max attempts for retry loops |
| `PANDORA_SANDBOX` | `none` | Sandbox level (deprecated; use `PANDORA_SANDBOX_LEVEL`) |
| `PANDORA_SANDBOX_LEVEL` | `none` | Sandbox enforcement level |
| `PANDORA_REGISTRY_URL` | `http://localhost:3001` | K-O-Palace registry server URL |
| `PANDORA_PROFILES_DIR` | `~/.pandora/profiles` | Execution profiles directory |
| `PANDORA_PROVIDER_POLICY` | (none) | Provider selection policy |
| `PANDORA_SECRET_KEY` | (none) | Ed25519 secret key for package signing |
| `PANDORA_TOKEN` | (none) | K-O-Palace authentication token |
| `PANDORA_LOGO` | (none) | Custom ASCII logo path |
| `OPENAI_API_KEY` | (none) | OpenAI API key |
| `ANTHROPIC_API_KEY` | (none) | Anthropic API key |
| `PANDORA_API_TOKEN` | (none) | Primary bearer token for the runtime API |
| `PANDORA_PROVIDER_API_KEY` | (none) | API key consumed by non-interactive `pandora setup` |
| `PANDORA_PAIRING_CODE` | (none) | Pairing code used to issue temporary remote-client tokens |
| `PANDORA_CREDENTIALS_KEY` | (none) | Fallback key for encrypted CLI credentials on Linux/headless environments; Windows and macOS prefer the OS credential store |
| `PANDORA_NODE_ID` | `local` | Stable node identifier returned by `/api/v1/node` |
| `PANDORA_NODE_NAME` | node ID | Human-readable remote node name |
| `PANDORA_RELEASE_BASE_URL` | (GitHub releases) | Optional release mirror URL for CLI install/update testing or private distribution |
| `PANDORA_EXECUTION_TIMEOUT_SECONDS` | `1800` | Maximum remote API execution duration; values outside `1..86400` use the default |
| PANDORA_DEV_MODE | (disabled) | Enables unauthenticated API only when set to 1, true, or yes |
| PANDORA_INSECURE | (disabled) | Explicitly bypasses API authentication only when set to 1, true, or yes; development use only |

## Data directories

`~/.pandora/` contains:

```
config.toml          — default settings
auth.json            — bootstrap tokens, API keys, sessions
model_registry.json  — cached model list from providers
connections/         — provider connection configs
sessions/            — execution session records
checkpoints/         — execution checkpoints for replay
artifacts/           — gene output artifacts
```

## Related

- [CLI reference](CLI.md)
- [Permissions](PERMISSIONS.md)
- [SDK guide](SDK.md)

## Persistent deny rules

`pandora deny add <pattern>` stores shell patterns in `config.toml` under `deny_shell_patterns`. Pandora loads these rules for every execution and denies matching tool inputs before they run. Use `pandora deny list` to inspect them and `pandora deny remove <pattern>` to remove one.

## Provider credentials

`pandora connection add ... --api-key` stores the key through `pandora-secrets`; `connections.toml` keeps only a reference such as `provider-openai`. Runtime provider loading resolves `PANDORA_SECRET_PROVIDER_OPENAI` first, then the OS credential store or encrypted fallback. Prefer environment injection for CI and avoid putting keys in shell history.
