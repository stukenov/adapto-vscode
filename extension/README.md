# Adapto for VS Code

Full IDE support for the [Adapto](https://github.com/stukenov/adapto-core) web framework.

![VS Code](https://img.shields.io/badge/VS%20Code-1.85%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

### Language Intelligence

- **Syntax Highlighting** — TextMate grammar for all 6 block types (route, script, template, style, resource, layout) with embedded Rust, CSS, and HTML
- **Diagnostics** — Real-time parse and compile errors via LSP
- **Autocomplete** — Context-aware completions for keywords, state fields, actions, event modifiers
- **Hover Info** — Documentation for keywords and type info for state/props/actions
- **Go to Definition** — Jump to state, prop, memo, and action declarations
- **Document Symbols** — Outline view with routes, state, actions, and forms
- **Semantic Highlighting** — Role-based coloring: state, props, memos, actions, decorators
- **Code Formatting** — Format `.adapto` files with proper block-aware indentation

### Developer Experience

- **Block Indicators** — Status bar shows which blocks are present in the active file
- **Control Flow Pairs** — Matched highlighting for `{#if}`/`{/if}`, `{#each}`/`{/each}`, etc.
- **Security Lens** — Inline annotations for `#[permission]` and `#[audit]` decorators
- **Block Separators** — Visual dividers between top-level blocks
- **Snippets** — 12 quick scaffolds: `apage`, `aroute`, `aif`, `aeach`, `aaction`, and more

### Color Themes

- **Adapto Night** — Dark theme with semantic role-based coloring
- **Adapto Day** — Light theme matching the same semantic design

### Tooling

- **CLI Integration** — Command Palette and status bar for `adapto dev`, `build`, `check`, `generate`
- **Project Sidebar** — Explorer showing routes, components, and resources
- **Live Preview** — WebView panel rendering template output in real-time

## Quick Start

1. Install the extension
2. Open a folder containing `.adapto` files
3. Syntax highlighting, diagnostics, and completions activate automatically
4. Press `Ctrl+Shift+P` → type "Adapto:" to access all commands

## Settings

All settings are under the `adapto.*` namespace. Open **Settings** → search "adapto".

| Setting | Default | Description |
|---------|---------|-------------|
| `adapto.lsp.enabled` | `true` | Enable the Language Server for diagnostics, completions, and hover |
| `adapto.lsp.path` | `""` | Custom path to the `adapto-lsp` binary (auto-detected if empty) |
| `adapto.preview.autoOpen` | `false` | Automatically open the preview panel when an `.adapto` file is opened |
| `adapto.format.onSave` | `false` | Format `.adapto` files on save |
| `adapto.devServer.port` | `3000` | Default port for the dev server |
| `adapto.blockIndicators.enabled` | `true` | Show block indicator icons in the status bar |
| `adapto.controlFlowHighlight.enabled` | `true` | Highlight matched control flow pairs |
| `adapto.securityLens.enabled` | `true` | Show inline annotations for security decorators |
| `adapto.blockSeparators.enabled` | `true` | Show visual separators between blocks |

## Requirements

- VS Code 1.85+
- [Adapto CLI](https://github.com/stukenov/adapto-core) (optional, for CLI commands)

## Development

```bash
# Build LSP server
cargo build -p adapto-lsp

# Build extension
cd extension && npm install && npm run build

# Launch Extension Development Host
# Press F5 in VS Code
```

## License

MIT
