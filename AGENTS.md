# AGENTS.md — construct-tui

Context for AI agents working in this repository.

---

## What is construct-tui?

Terminal UI client for Construct Messenger. Built with Rust + [Ratatui](https://ratatui.rs).
Runs on macOS, Linux, Raspberry Pi. Binary name: `konstrukt`.

Uses `construct-engine` (not `construct-core` directly) for all crypto and server comms.

---

## Architecture

```
main.rs
├── app.rs              — App state, event loop
├── engine_adapter.rs   — ConstructEngine wrapper (UiEvent dispatch, PlatformAction handler)
├── bridge.rs           — Bridge between TUI events and engine events
├── streaming.rs        — gRPC message stream handling
├── storage.rs          — Local message/session persistence
├── auth.rs             — Auth flow (registration, login)
├── invite.rs           — Invite link handling
├── orchestrator_task.rs — Background engine task
├── tui.rs              — Terminal setup / teardown
├── event.rs            — TUI input event types
└── screens/            — Screen views (chats, chat, settings, login, register…)
```

### engine_adapter.rs is the integration boundary

All interactions with the Construct protocol go through `EngineAdapter`.
It wraps `ConstructEngine` and translates TUI app events into `UiEvent`s,
and `PlatformAction`s back into TUI state updates.

---

## Build & Run

```bash
cargo build --release              # build — binary at target/release/konstrukt
cargo run                          # run in dev mode
cargo test                         # tests
cargo clippy                       # lint

# Install locally
cargo install --path .
```

Cross-compilation (for release):
```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu
# Linux aarch64 (Raspberry Pi)
cargo build --release --target aarch64-unknown-linux-gnu
```

---

## Key conventions

- All crypto/server operations go through `engine_adapter.rs` — never call `construct-engine` internals directly from screens
- TUI screens are in `screens/` — each screen is a Ratatui `Widget` impl
- State lives in `app.rs` `App` struct — screens borrow from it
- `config/` — user config directory (`~/.config/konstrukt/`)

---
---

## Shared Construct Docs Workflow

These instructions apply to GitHub Copilot, Codex, OpenCode, and similar coding agents.

### Division of labour — read this first

| Role | Tool | Responsibility |
|------|------|----------------|
| **Coding agent** (you) | Copilot / Codex | Write code + drop raw session notes into `wiki/sessions/` and `wiki/decisions/`. That is all. |
| **Wiki pipeline** | `obsidian-llm-wiki-local` (olw) | Reads `raw/`, synthesizes concepts, creates/updates wiki articles, generates cross-links. |
| **Developer** | Human + Obsidian | Reviews wiki draft articles, approves/rejects. Curates `raw/`. |

**Your job is code.** olw handles article synthesis. Write plain-markdown session notes; let the pipeline do the rest.

### Shared knowledge base

- Vault: `/Users/maximeliseyev/Code/construct-docs`
- `raw/` — source corpus. Do **not** rewrite or reorganize.
- `wiki/` — canonical curated knowledge base. **Read** from here before architectural work.
- `wiki/.drafts/` — **reserved for olw**. Never write here manually.
- `wiki/sessions/` — where coding agents write session notes.
- `wiki/decisions/` — where coding agents write long-lived decision records.

### Where to save durable reasoning

After any session involving architectural changes, design decisions, API changes, or non-obvious implementation choices:

1. **Always** create or update `wiki/sessions/YYYY-MM-DD-<topic>.md`.
2. **Always** fill in `# Why` — reasoning, alternatives considered, why rejected. Most important section.
3. If the decision constrains future work, also create `wiki/decisions/<topic>.md`.
4. Session notes: plain markdown, **no YAML frontmatter, no `[[wikilinks]]`** — olw adds those.

Required note sections: `# Context`, `# What Changed`, `# Why`, `# Intended Outcome`, `# Decisions`, `# Open Questions`

### Operational logging

Append a one-line entry to `wiki/log.md` after writing a note.
Format: `[YYYY-MM-DD HH:MM] note | <topic>`

