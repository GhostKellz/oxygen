use crate::utils::{
    format_duration, is_rust_project, output_json, output_text, run_command_with_timing,
};
use anyhow::Result;
use serde_json::json;
use tracing::{error, info};

pub async fn run(json_output: bool) -> Result<()> {
    if !is_rust_project() {
        let msg = "Not a Rust project (no Cargo.toml found)";
        if json_output {
            output_json(&json!({
                "error": msg,
                "success": false
            }));
        } else {
            error!("{}", msg);
        }
        return Ok(());
    }

    info!("Running Rust project checks...");

    let mut results = Vec::new();
    let mut all_passed = true;

    // Run cargo fmt --check
    info!("Running cargo fmt --check...");
    match run_command_with_timing("cargo", &["fmt", "--check"]) {
        Ok((output, duration)) => {
            let success = output.status.success();
            all_passed &= success;
            results.push(json!({
                "command": "cargo fmt --check",
                "success": success,
                "duration": format_duration(duration),
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr)
            }));

            if !json_output {
                if success {
                    output_text(&format!(
                        "✅ Format check passed ({})",
                        format_duration(duration)
                    ));
                } else {
                    output_text(&format!(
                        "❌ Format check failed ({})",
                        format_duration(duration)
                    ));
                    output_text(&String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        Err(e) => {
            all_passed = false;
            results.push(json!({
                "command": "cargo fmt --check",
                "success": false,
                "error": e.to_string()
            }));

            if !json_output {
                error!("❌ Failed to run cargo fmt: {}", e);
            }
        }
    }

    // Run cargo clippy
    info!("Running cargo clippy...");
    match run_command_with_timing("cargo", &["clippy", "--", "-D", "warnings"]) {
        Ok((output, duration)) => {
            let success = output.status.success();
            all_passed &= success;
            results.push(json!({
                "command": "cargo clippy",
                "success": success,
                "duration": format_duration(duration),
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr)
            }));

            if !json_output {
                if success {
                    output_text(&format!("✅ Clippy passed ({})", format_duration(duration)));
                } else {
                    output_text(&format!("❌ Clippy failed ({})", format_duration(duration)));
                    output_text(&String::from_utf8_lossy(&output.stdout));
                }
            }
        }
        Err(e) => {
            all_passed = false;
            results.push(json!({
                "command": "cargo clippy",
                "success": false,
                "error": e.to_string()
            }));

            if !json_output {
                error!("❌ Failed to run cargo clippy: {}", e);
            }
        }
    }

    // Run cargo check
    info!("Running cargo check...");
    match run_command_with_timing("cargo", &["check"]) {
        Ok((output, duration)) => {
            let success = output.status.success();
            all_passed &= success;
            results.push(json!({
                "command": "cargo check",
                "success": success,
                "duration": format_duration(duration),
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr)
            }));

            if !json_output {
                if success {
                    output_text(&format!("✅ Check passed ({})", format_duration(duration)));
                } else {
                    output_text(&format!("❌ Check failed ({})", format_duration(duration)));
                    output_text(&String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        Err(e) => {
            all_passed = false;
            results.push(json!({
                "command": "cargo check",
                "success": false,
                "error": e.to_string()
            }));

            if !json_output {
                error!("❌ Failed to run cargo check: {}", e);
            }
        }
    }

    if json_output {
        output_json(&json!({
            "success": all_passed,
            "results": results
        }));
    } else if all_passed {
        output_text("\n🎉 All checks passed!");
    } else {
        output_text("\n💥 Some checks failed!");
    }

    Ok(())
}
