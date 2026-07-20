# Configuration

Config file: ~/.pandora/config.toml

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
| PANDORA_HOME | ~/.pandora | Root data directory |
| PANDORA_DEFAULT_MODEL | (first) | Default model |
| PANDORA_DEFAULT_PROVIDER | (first) | Default provider |
| PANDORA_MAX_TOKENS | 4096 | Max tokens per execution |
| PANDORA_MAX_RETRIES | 3 | Max retries per step |
| PANDORA_SANDBOX | none | Sandbox level |
| OPENAI_API_KEY | (none) | OpenAI API key |
| ANTHROPIC_API_KEY | (none) | Anthropic API key |

## Data directories

~/.pandora/ contains: config.toml, auth.json, model_registry.json,
connections/, sessions/, checkpoints/, artifacts/

## Related

- [CLI reference](CLI.md)
- [Permissions](PERMISSIONS.md)
