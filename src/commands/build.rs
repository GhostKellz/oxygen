use crate::utils::{
    format_bytes, format_duration, get_binary_size, is_rust_project, output_json, output_text,
    run_command_with_timing,
};
use anyhow::Result;
use serde_json::json;
use std::path::Path;
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

    info!("Building Rust project...");

    match run_command_with_timing("cargo", &["build", "--release"]) {
        Ok((output, duration)) => {
            let success = output.status.success();
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Try to find the built binary
            let mut binary_info = None;
            
            if let Ok(cargo_toml) = std::fs::read_to_string("Cargo.toml") {
                if let Ok(manifest) = cargo_toml.parse::<toml::Value>() {
                    if let Some(package_name) = manifest
                        .get("package")
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                    {
                        // Try multiple possible paths for the binary
                        let possible_paths = [
                            format!("target/release/{}", package_name),
                            format!("target/x86_64-unknown-linux-gnu/release/{}", package_name),
                            format!("target/aarch64-unknown-linux-gnu/release/{}", package_name),
                        ];
                        
                        for path_str in &possible_paths {
                            let binary_path = Path::new(path_str);
                            if binary_path.exists() {
                                if let Ok(size) = get_binary_size(path_str) {
                                    binary_info = Some(json!({
                                        "path": path_str,
                                        "size_bytes": size,
                                        "size_formatted": format_bytes(size)
                                    }));
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if json_output {
                output_json(&json!({
                    "success": success,
                    "duration": format_duration(duration),
                    "binary": binary_info,
                    "stdout": stdout,
                    "stderr": stderr
                }));
            } else if success {
                output_text(&format!(
                    "✅ Build completed successfully in {}",
                    format_duration(duration)
                ));

                if let Some(binary) = binary_info {
                    if let (Some(path), Some(size)) =
                        (binary["path"].as_str(), binary["size_formatted"].as_str())
                    {
                        output_text(&format!("📦 Binary: {} ({})", path, size));
                    }
                }

                // Show any warnings
                if !stderr.is_empty() {
                    output_text("\n⚠️  Warnings:");
                    output_text(&stderr);
                }
            } else {
                output_text(&format!(
                    "❌ Build failed after {}",
                    format_duration(duration)
                ));
                if !stderr.is_empty() {
                    output_text(&stderr);
                }
                if !stdout.is_empty() {
                    output_text(&stdout);
                }
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "success": false,
                    "error": e.to_string()
                }));
            } else {
                error!("❌ Failed to run cargo build: {}", e);
            }
        }
    }

    Ok(())
}
