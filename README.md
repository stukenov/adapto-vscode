# Adapto for VS Code

Full IDE support for [Adapto](https://github.com/nickstukenov/adapto-core) `.adapto` files.

## Features

- **Syntax Highlighting** — Full TextMate grammar for all 6 block types with embedded Rust, CSS, and HTML
- **Diagnostics** — Real-time parse and compile errors via LSP
- **Autocomplete** — Context-aware completions for keywords, state fields, actions, event modifiers
- **Hover Info** — Documentation for keywords, type info for state/props/actions
- **Go to Definition** — Navigate to state, prop, memo, and action declarations
- **Document Symbols** — Outline view showing routes, state, actions, forms
- **Semantic Highlighting** — Rich coloring for state vs props vs actions vs decorators
- **Code Formatting** — Format .adapto files with proper indentation
- **Snippets** — Quick scaffolds: `apage`, `aroute`, `aif`, `aeach`, `aaction`, and more
- **CLI Integration** — Command palette and status bar for adapto dev/build/check/generate
- **Sidebar** — Project explorer showing routes, components, and resources
- **Live Preview** — WebView panel rendering template output

## Requirements

- VS Code 1.85+
- [Adapto CLI](https://github.com/nickstukenov/adapto-core) (for CLI commands)

## Quick Start

1. Install the extension
2. Open a folder containing `.adapto` files
3. Start editing — syntax highlighting, diagnostics, and completions activate automatically
4. Use `Ctrl+Shift+P` → "Adapto:" to access commands

## Development

```bash
# Build LSP server
cargo build -p adapto-lsp

# Build extension
cd extension && npm install && npm run build

# Run in VS Code
# Press F5 in VS Code to launch Extension Development Host
```

## License

MIT
