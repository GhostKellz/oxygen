use crate::utils::{output_json, output_text, run_command};
use anyhow::Result;
use serde_json::json;
use std::env;
use std::path::Path;
use tracing::info;

pub async fn run(json_output: bool) -> Result<()> {
    info!("Running environment diagnostics...");

    let mut checks = Vec::new();
    let mut all_good = true;

    // Check if rustc is available
    match run_command("rustc", &["--version"]) {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            checks.push(json!({
                "name": "Rust Compiler",
                "status": "ok",
                "value": version,
                "message": "rustc is available"
            }));
        }
        Err(_) => {
            all_good = false;
            checks.push(json!({
                "name": "Rust Compiler",
                "status": "error",
                "message": "rustc not found in PATH"
            }));
        }
    }

    // Check if cargo is available
    match run_command("cargo", &["--version"]) {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            checks.push(json!({
                "name": "Cargo",
                "status": "ok",
                "value": version,
                "message": "cargo is available"
            }));
        }
        Err(_) => {
            all_good = false;
            checks.push(json!({
                "name": "Cargo",
                "status": "error",
                "message": "cargo not found in PATH"
            }));
        }
    }

    // Check rustup
    match run_command("rustup", &["show"]) {
        Ok(output) => {
            let info = String::from_utf8_lossy(&output.stdout);
            let active_toolchain = info
                .lines()
                .find(|line| line.contains("active toolchain"))
                .map(|line| line.trim())
                .unwrap_or("unknown");

            checks.push(json!({
                "name": "Rustup",
                "status": "ok",
                "value": active_toolchain,
                "message": "rustup is available"
            }));
        }
        Err(_) => {
            checks.push(json!({
                "name": "Rustup",
                "status": "warning",
                "message": "rustup not found - toolchain management unavailable"
            }));
        }
    }

    // Check essential tools
    let tools = [
        ("clippy", "cargo clippy --version"),
        ("rustfmt", "cargo fmt --version"),
    ];

    for (tool_name, cmd) in &tools {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match run_command(parts[0], &parts[1..]) {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                checks.push(json!({
                    "name": format!("Tool: {}", tool_name),
                    "status": "ok",
                    "value": version,
                    "message": format!("{} is available", tool_name)
                }));
            }
            Err(_) => {
                all_good = false;
                checks.push(json!({
                    "name": format!("Tool: {}", tool_name),
                    "status": "error",
                    "message": format!("{} not available", tool_name)
                }));
            }
        }
    }

    // Check environment variables
    let env_vars = ["CARGO_HOME", "RUSTUP_HOME", "PATH"];
    for var in &env_vars {
        match env::var(var) {
            Ok(value) => {
                let status = if var == &"PATH" && !value.contains("cargo") {
                    "warning"
                } else {
                    "ok"
                };

                checks.push(json!({
                    "name": format!("Environment: {}", var),
                    "status": status,
                    "value": value,
                    "message": format!("{} is set", var)
                }));
            }
            Err(_) => {
                let status = if var == &"PATH" { "error" } else { "warning" };
                checks.push(json!({
                    "name": format!("Environment: {}", var),
                    "status": status,
                    "message": format!("{} is not set", var)
                }));

                if status == "error" {
                    all_good = false;
                }
            }
        }
    }

    // Check current directory
    if Path::new("Cargo.toml").exists() {
        checks.push(json!({
            "name": "Current Directory",
            "status": "ok",
            "message": "In a Rust project directory"
        }));
    } else {
        checks.push(json!({
            "name": "Current Directory",
            "status": "info",
            "message": "Not in a Rust project directory"
        }));
    }

    if json_output {
        output_json(&json!({
            "overall_status": if all_good { "healthy" } else { "issues_found" },
            "checks": checks
        }));
    } else {
        if all_good {
            output_text("🩺 Environment Health: ✅ Healthy");
        } else {
            output_text("🩺 Environment Health: ⚠️  Issues Found");
        }
        output_text("");

        for check in checks.iter() {
            let name = check["name"].as_str().unwrap_or("Unknown");
            let status = check["status"].as_str().unwrap_or("unknown");
            let message = check["message"].as_str().unwrap_or("");
            let value = check.get("value").and_then(|v| v.as_str()).unwrap_or("");

            let icon = match status {
                "ok" => "✅",
                "warning" => "⚠️ ",
                "error" => "❌",
                "info" => "ℹ️ ",
                _ => "❓",
            };

            if value.is_empty() {
                output_text(&format!("{} {}: {}", icon, name, message));
            } else {
                output_text(&format!("{} {}: {} ({})", icon, name, message, value));
            }
        }

        if !all_good {
            output_text("");
            output_text("💡 Suggestions:");
            output_text("   • Install missing tools with: rustup component add clippy rustfmt");
            output_text("   • Ensure Rust toolchain is properly installed via rustup.rs");
            output_text("   • Check that ~/.cargo/bin is in your PATH");
        }
    }

    Ok(())
}
