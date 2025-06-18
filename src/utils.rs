use anyhow::{Context, Result};
use serde_json::Value;
use std::process::Command;
use std::time::Instant;
use tracing::info;

pub fn run_command(cmd: &str, args: &[&str]) -> Result<std::process::Output> {
    info!("Running command: {} {}", cmd, args.join(" "));

    Command::new(cmd)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute command: {} {}", cmd, args.join(" ")))
}

pub fn run_command_with_timing(
    cmd: &str,
    args: &[&str],
) -> Result<(std::process::Output, std::time::Duration)> {
    let start = Instant::now();
    let output = run_command(cmd, args)?;
    let duration = start.elapsed();
    Ok((output, duration))
}

pub fn output_json(data: &Value) {
    println!("{}", serde_json::to_string_pretty(data).unwrap());
}

pub fn output_text(message: &str) {
    println!("{}", message);
}

pub fn is_rust_project() -> bool {
    std::path::Path::new("Cargo.toml").exists()
}

pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs_f64();
    if secs < 1.0 {
        format!("{:.0}ms", duration.as_millis())
    } else {
        format!("{:.2}s", secs)
    }
}

pub fn get_binary_size(path: &str) -> Result<u64> {
    let metadata =
        std::fs::metadata(path).with_context(|| format!("Failed to get metadata for {}", path))?;
    Ok(metadata.len())
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
