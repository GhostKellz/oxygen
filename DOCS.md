## 🧱 Docs

Oxygen uses `clap` under the hood with a modular structure:

```text
src/
├── main.rs              # CLI entrypoint
├── commands/
│   ├── build.rs
│   ├── check.rs
│   ├── doctor.rs
│   ├── env.rs
│   ├── info.rs
│   └── tools.rs
├── utils.rs             # Common helpers
└── config.rs            # Optional user config
```

All commands share a common logging layer (via `tracing`) and optionally support JSON output for CI tooling.

---

## ⚙️ Feature Scope

| Feature        | Status     | Notes                                  |
| -------------- | ---------- | -------------------------------------- |
| `oxy check`    | ✅ Stable   | Runs clippy + fmt + check in sequence  |
| `oxy build`    | ✅ Stable   | Time-tracked build, size summary       |
| `oxy doctor`   | 🧪 Beta    | Detects common environment/tool issues |
| `oxy tools`    | 🛠 Planned | List installed dev tools               |
| `oxy snapshot` | 🛠 Planned | Save state (build + git diff)          |
| `oxy init`     | 🔜 Planned | Wizard for new projects                |
| `oxy shell`    | 🔜 Planned | Spawns devshell with tool config       |

---

## 🧩 Plugin & Extension Vision

Oxygen is not plugin-based yet, but future extensibility is a design goal. Planned:

* `~/.config/oxygen/config.toml` for tool/alias customization
* Optional workspace-wide `.oxygen.toml`
* Support for project task aliases (like `just`) with native execution

---

## 📦 Build & Toolchain Compatibility

* Minimum Rust version: 1.74+
* Uses `clap`, `tracing`, `console`, `dirs`, `serde`
* Cross-platform: tested on Linux and macOS (WSL and Windows CLI support in roadmap)

---

## 🧪 Dev Tasks

```bash
cargo run -- check
cargo run -- build
cargo run -- doctor
```

For test runs:

```bash
cargo test
```

---

## 🌟 Goals Toward v0.1.0

*

