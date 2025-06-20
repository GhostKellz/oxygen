use crate::utils::{is_rust_project, output_json, output_text, run_command};
use crate::DepsAction;
use anyhow::{Result, anyhow};
use serde_json::json;
use std::collections::HashMap;
use tracing::info;

pub async fn run(action: DepsAction, json_output: bool) -> Result<()> {
    if !is_rust_project() {
        if json_output {
            output_json(&json!({
                "error": "Not in a Rust project directory",
                "is_rust_project": false
            }));
        } else {
            output_text("❌ Not in a Rust project (no Cargo.toml found)");
        }
        return Ok(());
    }

    match action {
        DepsAction::Tree => show_dependency_tree(json_output).await,
        DepsAction::Outdated => check_outdated_deps(json_output).await,
        DepsAction::Audit => audit_dependencies(json_output).await,
        DepsAction::Licenses => show_licenses(json_output).await,
        DepsAction::Size => analyze_dependency_sizes(json_output).await,
    }
}

async fn show_dependency_tree(json_output: bool) -> Result<()> {
    info!("Showing dependency tree...");

    match run_command("cargo", &["tree", "--format", "{p} {f}"]) {
        Ok(output) => {
            let tree_output = String::from_utf8_lossy(&output.stdout);
            
            if json_output {
                let dependencies = parse_dependency_tree(&tree_output);
                output_json(&json!({
                    "dependency_tree": dependencies,
                    "raw_output": tree_output.trim()
                }));
            } else {
                output_text("📦 Dependency Tree");
                output_text("==================");
                output_text(&tree_output);
            }
        }
        Err(_) => {
            if json_output {
                output_json(&json!({
                    "error": "cargo tree command failed",
                    "suggestion": "Make sure you're in a Rust project with dependencies"
                }));
            } else {
                output_text("❌ Failed to generate dependency tree");
                output_text("💡 Make sure you're in a Rust project with dependencies");
            }
            return Err(anyhow!("Failed to run cargo tree"));
        }
    }

    Ok(())
}

async fn check_outdated_deps(json_output: bool) -> Result<()> {
    info!("Checking for outdated dependencies...");

    match run_command("cargo", &["outdated", "--format", "json"]) {
        Ok(output) => {
            let outdated_output = String::from_utf8_lossy(&output.stdout);
            
            if json_output {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&outdated_output) {
                    output_json(&parsed);
                } else {
                    output_json(&json!({
                        "raw_output": outdated_output.trim()
                    }));
                }
            } else {
                output_text("📊 Outdated Dependencies");
                output_text("========================");
                
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&outdated_output) {
                    if let Some(dependencies) = parsed.get("dependencies").and_then(|d| d.as_array()) {
                        if dependencies.is_empty() {
                            output_text("✅ All dependencies are up to date!");
                        } else {
                            for dep in dependencies {
                                if let (Some(name), Some(project), Some(compat), Some(latest)) = (
                                    dep.get("name").and_then(|n| n.as_str()),
                                    dep.get("project").and_then(|p| p.as_str()),
                                    dep.get("compat").and_then(|c| c.as_str()),
                                    dep.get("latest").and_then(|l| l.as_str()),
                                ) {
                                    output_text(&format!("  {} {} → {} (latest: {})", name, project, compat, latest));
                                }
                            }
                        }
                    }
                } else {
                    output_text(&outdated_output);
                }
            }
        }
        Err(_) => {
            if json_output {
                output_json(&json!({
                    "error": "cargo outdated not available",
                    "suggestion": "Install with: cargo install cargo-outdated"
                }));
            } else {
                output_text("❌ cargo-outdated not installed");
                output_text("💡 Install with: cargo install cargo-outdated");
            }
        }
    }

    Ok(())
}

async fn audit_dependencies(json_output: bool) -> Result<()> {
    info!("Auditing dependencies for security issues...");

    match run_command("cargo", &["audit", "--format", "json"]) {
        Ok(output) => {
            let audit_output = String::from_utf8_lossy(&output.stdout);
            
            if json_output {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&audit_output) {
                    output_json(&parsed);
                } else {
                    output_json(&json!({
                        "raw_output": audit_output.trim()
                    }));
                }
            } else {
                output_text("🔒 Security Audit");
                output_text("================");
                
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&audit_output) {
                    if let Some(vulnerabilities) = parsed.get("vulnerabilities").and_then(|v| v.as_array()) {
                        if vulnerabilities.is_empty() {
                            output_text("✅ No known security vulnerabilities found!");
                        } else {
                            output_text(&format!("⚠️  Found {} vulnerability(ies):", vulnerabilities.len()));
                            for vuln in vulnerabilities {
                                if let (Some(package), Some(advisory)) = (
                                    vuln.get("package").and_then(|p| p.get("name")).and_then(|n| n.as_str()),
                                    vuln.get("advisory")
                                ) {
                                    let title = advisory.get("title").and_then(|t| t.as_str()).unwrap_or("Unknown");
                                    let severity = advisory.get("severity").and_then(|s| s.as_str()).unwrap_or("Unknown");
                                    output_text(&format!("  {} - {} ({})", package, title, severity));
                                }
                            }
                        }
                    }
                } else {
                    output_text(&audit_output);
                }
            }
        }
        Err(_) => {
            if json_output {
                output_json(&json!({
                    "error": "cargo audit not available",
                    "suggestion": "Install with: cargo install cargo-audit"
                }));
            } else {
                output_text("❌ cargo-audit not installed");
                output_text("💡 Install with: cargo install cargo-audit");
            }
        }
    }

    Ok(())
}

async fn show_licenses(json_output: bool) -> Result<()> {
    info!("Analyzing dependency licenses...");

    match run_command("cargo", &["tree", "--format", "{p} {l}"]) {
        Ok(output) => {
            let tree_output = String::from_utf8_lossy(&output.stdout);
            let mut license_counts: HashMap<String, u32> = HashMap::new();
            let mut dependencies = Vec::new();

            for line in tree_output.lines() {
                if let Some((name_version, license)) = line.trim().split_once(' ') {
                    if !license.is_empty() && license != "N/A" {
                        *license_counts.entry(license.to_string()).or_insert(0) += 1;
                        dependencies.push(json!({
                            "name": name_version,
                            "license": license
                        }));
                    }
                }
            }

            if json_output {
                output_json(&json!({
                    "dependencies": dependencies,
                    "license_summary": license_counts
                }));
            } else {
                output_text("📜 Dependency Licenses");
                output_text("=====================");
                
                if license_counts.is_empty() {
                    output_text("No license information found");
                } else {
                    output_text("License Summary:");
                    for (license, count) in &license_counts {
                        output_text(&format!("  {} - {} dependencies", license, count));
                    }
                    
                    output_text("");
                    output_text("Individual Dependencies:");
                    for dep in &dependencies {
                        let name = dep["name"].as_str().unwrap_or("unknown");
                        let license = dep["license"].as_str().unwrap_or("unknown");
                        output_text(&format!("  {} - {}", name, license));
                    }
                }
            }
        }
        Err(_) => {
            if json_output {
                output_json(&json!({
                    "error": "Failed to get license information",
                    "suggestion": "Make sure you're in a Rust project with dependencies"
                }));
            } else {
                output_text("❌ Failed to get license information");
                output_text("💡 Make sure you're in a Rust project with dependencies");
            }
        }
    }

    Ok(())
}

async fn analyze_dependency_sizes(json_output: bool) -> Result<()> {
    info!("Analyzing dependency sizes...");

    match run_command("cargo", &["bloat", "--release", "--crates"]) {
        Ok(output) => {
            let bloat_output = String::from_utf8_lossy(&output.stdout);
            
            if json_output {
                let size_analysis = parse_bloat_output(&bloat_output);
                output_json(&json!({
                    "size_analysis": size_analysis,
                    "raw_output": bloat_output.trim()
                }));
            } else {
                output_text("📊 Dependency Size Analysis");
                output_text("===========================");
                output_text(&bloat_output);
            }
        }
        Err(_) => {
            if json_output {
                output_json(&json!({
                    "error": "cargo bloat not available",
                    "suggestion": "Install with: cargo install cargo-bloat"
                }));
            } else {
                output_text("❌ cargo-bloat not installed");
                output_text("💡 Install with: cargo install cargo-bloat");
                output_text("   This tool helps identify which dependencies contribute most to binary size");
            }
        }
    }

    Ok(())
}

fn parse_dependency_tree(tree_output: &str) -> Vec<serde_json::Value> {
    let mut dependencies = Vec::new();
    
    for line in tree_output.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let depth = line.len() - trimmed.len();
            if let Some((name_version, features)) = trimmed.split_once(' ') {
                dependencies.push(json!({
                    "name": name_version,
                    "features": features,
                    "depth": depth / 4
                }));
            } else {
                dependencies.push(json!({
                    "name": trimmed,
                    "depth": depth / 4
                }));
            }
        }
    }
    
    dependencies
}

fn parse_bloat_output(bloat_output: &str) -> Vec<serde_json::Value> {
    let mut analysis = Vec::new();
    
    for line in bloat_output.lines() {
        if line.contains('%') && line.contains("KB") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                analysis.push(json!({
                    "percentage": parts[0],
                    "size": parts[1],
                    "crate": parts[2..].join(" ")
                }));
            }
        }
    }
    
    analysis
}