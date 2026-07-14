---
version: alpha
name: Pandora Systems
description: Constitutional AI operating substrate — replaceable services, source harnesses, meta harnesses, workflows, loops, and evolutionary cognition.
colors:
  primary: "#1A1C1E"
  secondary: "#4A5568"
  tertiary: "#B8422E"
  accent: "#2B6CB0"
  neutral: "#F7F5F2"
  surface: "#FFFFFF"
  code: "#1E293B"
  success: "#38A169"
  warning: "#D69E2E"
  error: "#E53E3E"
typography:
  h1:
    fontFamily: "Inter"
    fontSize: 3rem
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: "-0.02em"
  h2:
    fontFamily: "Inter"
    fontSize: 2rem
    fontWeight: 600
    lineHeight: 1.2
  h3:
    fontFamily: "Inter"
    fontSize: 1.5rem
    fontWeight: 600
    lineHeight: 1.3
  body-lg:
    fontFamily: "Inter"
    fontSize: 1.125rem
    fontWeight: 400
    lineHeight: 1.6
  body-md:
    fontFamily: "Inter"
    fontSize: 1rem
    lineHeight: 1.6
  body-sm:
    fontFamily: "Inter"
    fontSize: 0.875rem
    lineHeight: 1.5
  code:
    fontFamily: "JetBrains Mono"
    fontSize: 0.875rem
    lineHeight: 1.5
rounded:
  sm: 4px
  md: 8px
  lg: 12px
spacing:
  xs: 4px
  sm: 8px
  md: 16px
  lg: 24px
  xl: 32px
  xxl: 48px
components:
  code-block:
    backgroundColor: "{colors.code}"
    textColor: "#E2E8F0"
    rounded: "{rounded.md}"
  link:
    textColor: "{colors.accent}"
  link-hover:
    textColor: "{colors.tertiary}"
  blockquote:
    borderLeft: 4px
    borderColor: "{colors.tertiary}"
---

## Overview

Pandora Systems is a constitutional AI operating substrate. The design identity reflects engineering precision, constitutional seriousness, and architectural clarity. The palette is deliberately restrained — deep ink for structure, warm accent for action, and generous whitespace for readability.

## Colors

- **Primary (#1A1C1E):** Deep ink for headlines, navigation, and structural elements.
- **Secondary (#4A5568):** Muted slate for secondary text, captions, and metadata.
- **Tertiary (#B8422E):** Warm rust — the sole interaction color. Used sparingly for emphasis, links, and calls to action.
- **Accent (#2B6CB0):** Technical blue for code references, APIs, and terminal output.
- **Neutral (#F7F5F2):** Warm off-white for page backgrounds. Reduces eye strain compared to pure white.
- **Surface (#FFFFFF):** Pure white for cards, sidebars, and elevated surfaces.
- **Success/Error/Warning:** Functional colors for test results, build status, and governance outcomes.

## Typography

Inter is the primary typeface — neutral, highly readable, and optimized for screens at all sizes. JetBrains Mono is used exclusively for code blocks, terminal output, and CLI examples.

## Layout & Spacing

The grid is 8px-based. Content max-width should not exceed 800px for readability. Code blocks and sidebars may extend to 1,200px. Vertical rhythm follows a 4px baseline.

## Components

- **Code blocks** use a dark slate background with light syntax highlighting.
- **Links** are accent blue by default, transitioning to tertiary rust on hover.
- **Blockquotes** have a 4px warm rust left border with no background fill.

## Do's and Don'ts

- Do use the tertiary color sparingly — it loses impact when overused.
- Don't use pure black (#000) for text; use primary (#1A1C1E).
- Do prefer the warm neutral background over pure white for long-form reading.
- Don't mix accent blue with tertiary rust — choose one interaction color per section.
