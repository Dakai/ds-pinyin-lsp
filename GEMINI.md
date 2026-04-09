# ds-pinyin-lsp Project Context

## Project Overview

`ds-pinyin-lsp` is an LSP-based Pinyin input method designed for (neo)vim and other LSP-capable editors. Its primary goal is to allow users to input Chinese characters without switching system input methods, thereby avoiding common frustrations like accidental input method activation in Normal mode.

### Architecture

The project is structured as a monorepo with three main components:

1.  **`packages/ds-pinyin-lsp`**: The core Language Server Protocol (LSP) implementation in Rust. It uses `tower-lsp` for the LSP framework and `rusqlite` to query a SQLite dictionary database.
2.  **`packages/coc-ds-pinyin`**: A `coc.nvim` extension (TypeScript) that manages the LSP server's lifecycle, handles configuration, and provides status bar integration.
3.  **`packages/dict-builder`**: A Rust-based utility to build the `dict.db3` SQLite database from external dictionary files (primarily from the `rime-ice` project).

## Building and Running

### 1. Dictionary Builder (`packages/dict-builder`)
- **Download Dictionaries**: Run `sh download.sh` to fetch dictionary files from `rime-ice`.
- **Build Database**: Run `cargo run --release` to generate `dicts/dict.db3`.

### 2. LSP Server (`packages/ds-pinyin-lsp`)
- **Build**: Run `cargo build --release` in the package directory.
- **Binary**: The resulting binary is typically used by the editor extension or configured manually in the LSP client.

### 3. coc.nvim Extension (`packages/coc-ds-pinyin`)
- **Install Dependencies**: `yarn install` or `npm install`.
- **Build**: `npm run build` (uses `esbuild`).
- **Development**: `npm run watch`.
- **Automated Deployment**: The extension is capable of downloading the `ds-pinyin-lsp` binary and `dict.db3` database from GitHub releases if they are not found or configured. It also performs update checks on startup.

## Development Conventions

- **Language Support**: Core logic and performance-sensitive parts (LSP, dictionary querying) are in Rust. Extension and editor integration are in TypeScript.
- **Database**: Uses SQLite for fast pinyin-to-character lookups. The schema includes a `dict` table with `pinyin`, `hanzi`, and `priority` columns.
- **LSP Features**:
    - `textDocument/completion`: Main entry point for pinyin suggestions.
    - `$/turn/completion`: Custom notification to toggle completion on/off.
- **Configuration**: Managed via LSP `initializationOptions`. Key settings include `db_path`, `completion_around_mode`, and `match_long_input`.

## Key Files

- `README.md`: Root documentation with usage instructions and configuration details.
- `packages/ds-pinyin-lsp/src/lsp.rs`: Main LSP server logic.
- `packages/ds-pinyin-lsp/src/sqlite.rs`: Database query implementations.
- `packages/coc-ds-pinyin/src/index.ts`: coc.nvim extension entry point.
- `packages/dict-builder/src/main.rs`: Dictionary processing and DB creation logic.
