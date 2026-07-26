# pandora/design-domain

UI/UX design — brand identity, color theory, typography, motion, accessibility.

## What It Does

A domain harness for design tasks:
- **Design Review** — review a design for consistency, patterns, issues
- **Brand Identity** — suggest brand identity elements
- **Color Theory** — suggest color palette from base color
- **Typography** — suggest typography pairings
- **Motion Design** — suggest animation patterns
- **UI Patterns** — suggest UI component patterns
- **Accessibility** — check accessibility compliance

## Install

```bash
pandora-kuber install pandora/design-domain
```

## Usage

```bash
# Review a design
pandora run design-review "https://example.com"

# Suggest brand identity
pandora run brand-identity "tech startup"

# Suggest color palette
pandora run color-theory "#3498db"

# Check accessibility
pandora run accessibility "index.html"
```

## Genes

| Gene | Description |
|------|-------------|
| `design-review` | Review a design for consistency, patterns, issues |
| `brand-identity` | Suggest brand identity elements |
| `color-theory` | Suggest color palette from base color |
| `typography` | Suggest typography pairings |
| `motion-design` | Suggest animation patterns |
| `ui-patterns` | Suggest UI component patterns |
| `accessibility` | Check accessibility compliance |

## Slash Commands

| Command | Description |
|---------|-------------|
| `/design.review` | Review a design: /design.review [url\|file] |
| `/design.brand` | Suggest brand identity elements |
| `/design.color` | Suggest color palette from base color |
| `/design.typography` | Suggest typography pairings |
| `/design.motion` | Suggest animation patterns |
| `/design.ui` | Suggest UI component patterns |
| `/design.a11y` | Check accessibility compliance |

## Permissions

| Allow | Deny |
|-------|------|
| `filesystem.read` | `shell.execute` |
| | `filesystem.write` |
| | `network.external` |

## Trust Level

Requires `publisher-verified` packages. Source code must be available.

## License

Apache-2.0
