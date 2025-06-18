# Oxygen

> The essential Rust dev environment enhancer

![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![Tool Type](https://img.shields.io/badge/type-Developer%20Tool-blue)
![Platform](https://img.shields.io/badge/platform-linux--mac--windows-success)
![Status](https://img.shields.io/badge/stage-active-important)

**Oxygen** (`oxy`) is a clean, fast, and extendable CLI utility designed to make Rust development smoother, smarter, and more efficient — without replacing Cargo.

Whether you're hacking on side projects, managing a clean dotfile stack, or maintaining complex dev workflows across systems, Oxygen acts as your essential sidekick.

---

## 🚀 Features

### ⚙️ Cargo Enhancements

* `oxy build`: Build with enhanced timing and size summaries
* `oxy check`: Run `clippy`, `fmt`, and `cargo check` together
* `oxy test --watch`: Auto-retest failing tests
* `oxy clean --deep`: Remove build cache, target, stale deps

### 🧰 DevOps Tools

* `oxy doctor`: Diagnose broken rustup paths, toolchain mismatches, env issues
* `oxy env`: Summarize current Rust environment, toolchains, config
* `oxy tools`: Inspect installed tools (`rustfmt`, `zls`, `lldb`, etc.)

### 📦 Project Intelligence

* `oxy info`: Show project metadata, git status, Cargo.toml preview
* `oxy size`: Show binary size breakdowns and compile stats
* `oxy snapshot`: Save current dev state as a snapshot (build + git diff)

### 🖥 Workflow Utilities

* `oxy init`: Project scaffolding wizard (custom or cargo)
* `oxy shell`: Start a dev shell with tools + paths preloaded
* `oxy run`: Run with profiling/stats overlays

---

## 🧠 Why Oxygen?

* **Rust-first:** Built in Rust, for Rust
* **Cargo-aligned:** Never overrides cargo — enhances it
* **Cross-platform:** Linux, macOS, and Windows-ready
* **Lightweight:** Single binary, no deps
* **Extensible:** Add your own workflows, aliases, and build scripts

---

## 🔧 Installation (Coming Soon)

```bash
cargo install oxygen-cli
```

or clone and build:

```bash
git clone https://github.com/yourname/oxygen
cd oxygen
cargo build --release
```

---

## 📝 Usage Examples

```bash
oxy check           # Runs clippy + fmt + check
oxy build           # Builds and shows size/timing info
oxy run --profile   # Runs with perf overlay
oxy tools           # Lists installed Rust-related dev tools
oxy doctor          # Diagnoses common environment issues
```

---

Stay fast. Stay focused. Stay Rusty. 🦀

