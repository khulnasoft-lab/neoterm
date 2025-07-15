# ✅ NeoPilot Terminal — Full Feature Roadmap & To-Do

> A complete modern terminal architecture with AI, workflows, plugin support, language adaptation, and cloud sync.

---

## 🧱 Core Engine

- [ ] `command/`: Shell lifecycle & PTY I/O
- [ ] `string_offset/`: Unicode-aware text slicing/indexing
- [ ] `sum_tree/`: Undo/redo history with tree structure
- [ ] `syntax_tree/`: Shell/code syntax parser
- [ ] `virtual_fs/`: Sandboxed command execution
- [ ] `watcher/`: File system + command runtime monitoring

---

## 🖥️ UI + Terminal Features

- [ ] Command blocks (status: Running, Done, Error)
- [ ] Input history (↑ ↓)
- [ ] Scrollable block view
- [ ] Collapsible output blocks
- [ ] Command palette (⌘K)
- [ ] Fuzzy finder for files/commands
- [ ] Tabbed sessions
- [ ] Custom themes (light/dark/custom)
- [ ] Rounded corners, font config, padding
- [ ] GPU acceleration with `wgpu`
- [ ] Notifications system

---

## 🔍 Search + Interaction

- [ ] `fuzzy_match/`: Command & block fuzzy matching
- [ ] `mcq/`: Multi-choice prompts (fuzzy UI)
- [ ] `markdown_parser/`: Markdown/rich text block output
- [ ] Keybinding remapper
- [ ] File explorer panel
- [ ] Block jump (Ctrl+J / Ctrl+K)

---

## 🧠 AI Integration

- [ ] `agent_mode_eval/`: Wrap terminal context into AI task
- [ ] `lpc/`: Cross-shell command translation (bash ↔ pwsh)
- [ ] `languages/`: Shell/language detection + switching
- [ ] `natural_language_detection/`: Detect & adapt user input language
- [ ] AI Assistant sidebar (OpenAI, Claude, or local model)
- [ ] "Explain this output" button
- [ ] AI command auto-fix + smart suggestions
- [ ] Local model support (`ollama`, `llama.cpp`)
- [ ] Context injection: cwd, env, history

---

## 📦 Workflows & Cloud Sync

- [ ] `asset_macro/`: Reusable command macros/workflows
- [ ] `drive/`: Warp Drive clone — store workflows & prefs
- [ ] `graphql/`: Expose local workflows via API
- [ ] `virtual_fs/`: Replay workflows safely in sandbox
- [ ] Workflow manager: create/edit/execute/save
- [ ] Team workflow sharing
- [ ] SQLite + cloud sync (Supabase or Firebase)

---

## 🧩 Plugins & Extensibility

- [ ] `serve_wasm/`: WASM-based plugin runtime
- [ ] Plugin manifest format (JSON)
- [ ] Plugin manager sidebar
- [ ] `mlua`: Lua scripting engine for plugins
- [ ] Runtime hooks: pre-command, post-output
- [ ] Hot reload plugin system
- [ ] `resources/`: Icons, manifests, plugin assets

---

## 🌐 Integrations

- [ ] `integration/`: Git, Docker, SSH, GitHub CLI
- [ ] `websocket/`: Real-time bi-directional events
- [ ] SSH session manager
- [ ] Remote session support
- [ ] Terminal state sync across devices

---

## 🔧 Dev Infra & Tooling

- [ ] Unit tests for all modules
- [ ] UI snapshot testing
- [ ] Benchmark PTY, AI, fuzzy search
- [ ] GitHub Actions CI (build, lint, test)
- [ ] DevContainer + Dockerfile
- [ ] Installable binary/AppImage

---

## 📁 Suggested Module Status (by Folder)

| Module                      | Status         |
|----------------------------|----------------|
| `command/`                 | ⬜ Not Started  |
| `agent_mode_eval/`         | ⬜ Not Started  |
| `fuzzy_match/`             | ⬜ Not Started  |
| `lpc/`                     | ⬜ Not Started  |
| `mcq/`                     | ⬜ Not Started  |
| `natural_language_detection/` | ⬜ Not Started  |
| `sum_tree/`                | ⬜ Not Started  |
| `serve_wasm/`              | ⬜ Not Started  |
| `drive/`                   | ⬜ Not Started  |
| `graphql/`                 | ⬜ Not Started  |
| `asset_macro/`             | ⬜ Not Started  |
| `syntax_tree/`             | ⬜ Not Started  |
| `string_offset/`           | ⬜ Not Started  |
| `resources/`               | ⬜ Not Started  |
| `virtual_fs/`              | ⬜ Not Started  |
| `watcher/`                 | ⬜ Not Started  |
| `integration/`             | ⬜ Not Started  |
| `websocket/`               | ⬜ Not Started  |
| `languages/`               | ⬜ Not Started  |
| `markdown_parser/`         | ⬜ Not Started  |

---

## 📦 Bonus Ideas (Advanced Features)

- [ ] Voice input (whisper.cpp)
- [ ] Vision-to-terminal (screenshot → command)
- [ ] AI inline code assist in terminal blocks
- [ ] Terminal notebook mode
- [ ] Warp Drive on the Web (remote UI)
- [ ] Environment variables manager
- [ ] Model context protocol (for agents)
- [ ] Team terminal session sharing

---

## 📦 Export Options

Would you like:
- 📁 `TODO.md` file (downloadable)?
- 🗂️ ZIP scaffold with `src/` + each module folder created?
- ✅ GitHub Project Template board?

Let me know and I’ll generate it instantly!
