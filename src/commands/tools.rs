use crate::utils::{output_json, output_text, run_command};
use anyhow::Result;
use serde_json::json;
use tracing::info;

pub async fn run(json_output: bool) -> Result<()> {
    info!("Scanning for Rust development tools...");

    let tools = [
        ("rustc", "rustc --version"),
        ("cargo", "cargo --version"),
        ("rustfmt", "rustfmt --version"),
        ("clippy", "cargo clippy --version"),
        ("rustup", "rustup --version"),
        ("cargo-watch", "cargo watch --version"),
        ("cargo-edit", "cargo add --version"),
        ("cargo-audit", "cargo audit --version"),
        ("cargo-outdated", "cargo outdated --version"),
        ("cargo-tree", "cargo tree --version"),
        ("cargo-expand", "cargo expand --version"),
        ("cargo-flamegraph", "cargo flamegraph --version"),
        ("cargo-criterion", "cargo criterion --version"),
        ("rust-analyzer", "rust-analyzer --version"),
        ("rls", "rls --version"),
        ("gdb", "gdb --version"),
        ("lldb", "lldb --version"),
        ("valgrind", "valgrind --version"),
    ];

    let mut found_tools = Vec::new();
    let mut missing_tools = Vec::new();

    for (name, cmd) in &tools {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match run_command(parts[0], &parts[1..]) {
            Ok(output) => {
                let version_info = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .unwrap_or("unknown version")
                    .trim()
                    .to_string();

                found_tools.push(json!({
                    "name": name,
                    "version": version_info,
                    "status": "available"
                }));
            }
            Err(_) => {
                missing_tools.push(json!({
                    "name": name,
                    "status": "not_found"
                }));
            }
        }
    }

    if json_output {
        output_json(&json!({
            "found_tools": found_tools,
            "missing_tools": missing_tools,
            "summary": {
                "total_found": found_tools.len(),
                "total_missing": missing_tools.len()
            }
        }));
    } else {
        output_text("🔧 Rust Development Tools");
        output_text("==========================");
        output_text(&format!(
            "Found: {} tools | Missing: {} tools",
            found_tools.len(),
            missing_tools.len()
        ));
        output_text("");

        if !found_tools.is_empty() {
            output_text("✅ Available Tools:");
            for tool in &found_tools {
                let name = tool["name"].as_str().unwrap_or("unknown");
                let version = tool["version"].as_str().unwrap_or("unknown");
                output_text(&format!("  {} - {}", name, version));
            }
        }

        if !missing_tools.is_empty() {
            output_text("");
            output_text("❌ Missing Tools:");
            for tool in &missing_tools {
                let name = tool["name"].as_str().unwrap_or("unknown");
                output_text(&format!("  {}", name));
            }

            output_text("");
            output_text("💡 Installation suggestions:");
            output_text("  • cargo install cargo-watch cargo-edit cargo-audit cargo-outdated");
            output_text("  • cargo install cargo-expand flamegraph cargo-criterion");
            output_text(
                "  • Install rust-analyzer via your editor or rustup component add rust-analyzer",
            );
        }
    }

    Ok(())
}
