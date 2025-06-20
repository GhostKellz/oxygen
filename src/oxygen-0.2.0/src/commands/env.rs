use crate::utils::{output_json, output_text, run_command};
use anyhow::Result;
use serde_json::json;
use std::env;
use tracing::info;

pub async fn run(json_output: bool) -> Result<()> {
    info!("Gathering Rust environment information...");

    let mut env_info = json!({});

    // Get Rust version
    if let Ok(output) = run_command("rustc", &["--version"]) {
        env_info["rust_version"] = json!(String::from_utf8_lossy(&output.stdout).trim());
    }

    // Get Cargo version
    if let Ok(output) = run_command("cargo", &["--version"]) {
        env_info["cargo_version"] = json!(String::from_utf8_lossy(&output.stdout).trim());
    }

    // Get rustup info
    if let Ok(output) = run_command("rustup", &["show"]) {
        let info = String::from_utf8_lossy(&output.stdout);
        if let Some(active) = info.lines().find(|line| line.contains("active toolchain")) {
            env_info["active_toolchain"] = json!(active.trim());
        }

        let toolchains: Vec<&str> = info
            .lines()
            .skip_while(|line| !line.contains("installed toolchains"))
            .skip(1)
            .take_while(|line| !line.is_empty() && !line.contains("active toolchain"))
            .collect();
        env_info["installed_toolchains"] = json!(toolchains);
    }

    // Get target info
    if let Ok(output) = run_command("rustc", &["--print", "target-list"]) {
        let target_output = String::from_utf8_lossy(&output.stdout);
        let targets: Vec<&str> = target_output
            .lines()
            .take(10) // Limit output
            .collect();
        env_info["available_targets"] = json!(targets);
    }

    // Get current target
    if let Ok(output) = run_command("rustc", &["-vV"]) {
        let info = String::from_utf8_lossy(&output.stdout);
        if let Some(host) = info.lines().find(|line| line.starts_with("host:")) {
            env_info["host_target"] = json!(host.trim_start_matches("host: "));
        }
    }

    // Environment variables
    let important_vars = ["CARGO_HOME", "RUSTUP_HOME", "RUST_BACKTRACE"];
    let mut env_vars = json!({});
    for var in &important_vars {
        if let Ok(value) = env::var(var) {
            env_vars[*var] = json!(value);
        }
    }
    env_info["environment"] = env_vars;

    if json_output {
        output_json(&env_info);
    } else {
        output_text("🦀 Rust Environment Summary");
        output_text("================================");

        if let Some(rust_ver) = env_info["rust_version"].as_str() {
            output_text(&format!("Rust: {}", rust_ver));
        }

        if let Some(cargo_ver) = env_info["cargo_version"].as_str() {
            output_text(&format!("Cargo: {}", cargo_ver));
        }

        if let Some(toolchain) = env_info["active_toolchain"].as_str() {
            output_text(&format!("Active Toolchain: {}", toolchain));
        }

        if let Some(host) = env_info["host_target"].as_str() {
            output_text(&format!("Host Target: {}", host));
        }

        output_text("");
        output_text("Environment Variables:");
        if let Some(env_vars) = env_info["environment"].as_object() {
            for (key, value) in env_vars {
                output_text(&format!("  {}: {}", key, value.as_str().unwrap_or("N/A")));
            }
        }
    }

    Ok(())
}
