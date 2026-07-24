# Configuration

Config file: `~/.pandora/config.toml`

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
| `PANDORA_SANDBOX` | `none` | Sandbox level (deprecated, use `PANDORA_SANDBOX_LEVEL`) |
| `PANDORA_SANDBOX_LEVEL` | `none` | Sandbox enforcement level |
| `PANDORA_PALACE_URL` | `http://localhost:3000` | K-O Palace registry server URL |
| `PANDORA_PROFILES_DIR` | `~/.pandora/profiles` | Execution profiles directory |
| `PANDORA_PROVIDER_POLICY` | (none) | Provider selection policy |
| `PANDORA_SECRET_KEY` | (none) | Ed25519 secret key for package signing |
| `PANDORA_TOKEN` | (none) | K-O Palace authentication token |
| `PANDORA_LOGO` | (none) | Custom ASCII logo path |
| `OPENAI_API_KEY` | (none) | OpenAI API key |
| `ANTHROPIC_API_KEY` | (none) | Anthropic API key |

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
