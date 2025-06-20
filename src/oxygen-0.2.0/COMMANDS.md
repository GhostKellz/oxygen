# Oxygen — COMMANDS.md

This document provides a detailed reference for all available `oxy` CLI commands.

---

## 🧰 General Commands

### `oxy help`

* Show global help and usage information.

### `oxy version`

* Display Oxygen CLI version and platform metadata.

---

## ⚙️ Cargo Workflow

### `oxy check`

* Runs `cargo fmt`, `cargo clippy`, and `cargo check` in sequence.
* Outputs pass/fail and lint summary.

### `oxy build`

* Executes `cargo build` with enhanced output.
* Shows binary size, build time, and success/failure status.

### `oxy clean [--deep]`

* Clean standard Cargo `target` dir.
* `--deep`: also clears build cache, `.cargo`, stale artifacts.

### `oxy test [--watch]`

* Run `cargo test` normally.
* `--watch`: rerun tests on file save using `cargo-watch` if installed.

### `oxy run [--profile]`

* Run project binary with optional runtime metrics (time, memory).
* `--profile`: enables system-level profiling integration.

---

## 📦 Project & Metadata

### `oxy info`

* Displays project name, version, Cargo.toml metadata.
* Includes Git branch and dirty status if available.

### `oxy size`

* Calculates binary size, sections (like `.text`, `.data`), and compile artifacts.

### `oxy snapshot`

* Saves snapshot of current git diff + build meta.
* Optional support for saving snapshot in a workspace `.oxygen/snapshots/` dir.

---

## 🧠 Developer Environment Tools

### `oxy doctor`

* Diagnose broken toolchains, environment PATH issues, or rust-analyzer problems.
* Recommends fixes for common setup issues.

### `oxy env`

* Display current `$PATH`, Cargo home, default toolchain, and active shell.

### `oxy tools`

* List currently installed developer tools such as `rustfmt`, `zls`, `lldb`, `cargo-nextest`, etc.
* Supports JSON or table output.

### `oxy shell`

* Launch a shell session with Rust tools and env loaded from `.oxygenrc` or workspace defaults.

---

## 🧪 Experimental & Planned

### `oxy init`

* Create a new project scaffold with optional presets.
* Customizes `Cargo.toml`, `src/main.rs`, and Git init.

### `oxy alias`

* Define custom command aliases in `.oxygen.toml`.
* Plan to support `oxy alias fmt-check = "cargo fmt && cargo clippy"`

### `oxy config`

* View or edit Oxygen config files in CLI (planned).
* Support for global + per-project configuration.

---

Stay fast. Stay focused. Stay Rusty. 🦀

