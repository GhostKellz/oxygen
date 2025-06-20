use crate::utils::{output_json, output_text, run_command};
use crate::GpgAction;
use anyhow::{Result, anyhow};
use serde_json::json;
use std::path::Path;
use tracing::info;

pub async fn run(action: GpgAction, json_output: bool) -> Result<()> {
    match action {
        GpgAction::Sign { target } => sign_target(&target, json_output).await,
        GpgAction::Verify { target } => verify_target(&target, json_output).await,
        GpgAction::Setup => setup_gpg_for_rust(json_output).await,
    }
}

async fn sign_target(target: &str, json_output: bool) -> Result<()> {
    info!("Signing target: {}", target);

    match target {
        "commit" => sign_commit(json_output).await,
        "tag" => sign_latest_tag(json_output).await,
        _ => {
            // Assume it's a file path
            sign_file(target, json_output).await
        }
    }
}

async fn sign_commit(json_output: bool) -> Result<()> {
    // Check if we're in a git repository
    if !Path::new(".git").exists() {
        if json_output {
            output_json(&json!({
                "error": "Not in a git repository",
                "action": "sign_commit"
            }));
        } else {
            output_text("❌ Not in a git repository");
        }
        return Err(anyhow!("Not in a git repository"));
    }

    // Check if GPG signing is configured
    match run_command("git", &["config", "user.signingkey"]) {
        Ok(output) => {
            let signing_key = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            // Create a signed commit
            match run_command("git", &["commit", "--amend", "--no-edit", "-S"]) {
                Ok(_) => {
                    if json_output {
                        output_json(&json!({
                            "action": "sign_commit",
                            "status": "success",
                            "signing_key": signing_key
                        }));
                    } else {
                        output_text("✅ Successfully signed the last commit");
                        output_text(&format!("🔑 Using key: {}", signing_key));
                    }
                }
                Err(e) => {
                    if json_output {
                        output_json(&json!({
                            "action": "sign_commit",
                            "status": "error",
                            "error": e.to_string()
                        }));
                    } else {
                        output_text("❌ Failed to sign commit");
                        output_text(&format!("Error: {}", e));
                    }
                    return Err(anyhow!("Failed to sign commit: {}", e));
                }
            }
        }
        Err(_) => {
            if json_output {
                output_json(&json!({
                    "error": "GPG signing not configured",
                    "action": "sign_commit",
                    "suggestion": "Run 'oxy gpg setup' to configure GPG signing"
                }));
            } else {
                output_text("❌ GPG signing not configured for git");
                output_text("💡 Run 'oxy gpg setup' to configure GPG signing");
            }
            return Err(anyhow!("GPG signing not configured"));
        }
    }

    Ok(())
}

async fn sign_latest_tag(json_output: bool) -> Result<()> {
    if !Path::new(".git").exists() {
        if json_output {
            output_json(&json!({
                "error": "Not in a git repository",
                "action": "sign_tag"
            }));
        } else {
            output_text("❌ Not in a git repository");
        }
        return Err(anyhow!("Not in a git repository"));
    }

    // Get the latest tag
    match run_command("git", &["describe", "--tags", "--abbrev=0"]) {
        Ok(output) => {
            let tag_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            // Sign the tag
            match run_command("git", &["tag", "-s", &tag_name, "-f", "-m", &format!("Signed tag {}", tag_name)]) {
                Ok(_) => {
                    if json_output {
                        output_json(&json!({
                            "action": "sign_tag",
                            "status": "success",
                            "tag": tag_name
                        }));
                    } else {
                        output_text(&format!("✅ Successfully signed tag: {}", tag_name));
                    }
                }
                Err(e) => {
                    if json_output {
                        output_json(&json!({
                            "action": "sign_tag",
                            "status": "error",
                            "tag": tag_name,
                            "error": e.to_string()
                        }));
                    } else {
                        output_text(&format!("❌ Failed to sign tag: {}", tag_name));
                        output_text(&format!("Error: {}", e));
                    }
                    return Err(anyhow!("Failed to sign tag: {}", e));
                }
            }
        }
        Err(_) => {
            if json_output {
                output_json(&json!({
                    "error": "No tags found",
                    "action": "sign_tag"
                }));
            } else {
                output_text("❌ No tags found in repository");
            }
            return Err(anyhow!("No tags found"));
        }
    }

    Ok(())
}

async fn sign_file(file_path: &str, json_output: bool) -> Result<()> {
    if !Path::new(file_path).exists() {
        if json_output {
            output_json(&json!({
                "error": "File not found",
                "file": file_path,
                "action": "sign_file"
            }));
        } else {
            output_text(&format!("❌ File not found: {}", file_path));
        }
        return Err(anyhow!("File not found"));
    }

    let signature_path = format!("{}.sig", file_path);

    match run_command("gpg", &["--detach-sign", "--armor", "--output", &signature_path, file_path]) {
        Ok(_) => {
            if json_output {
                output_json(&json!({
                    "action": "sign_file",
                    "status": "success",
                    "file": file_path,
                    "signature": signature_path
                }));
            } else {
                output_text(&format!("✅ Successfully signed file: {}", file_path));
                output_text(&format!("📝 Signature saved to: {}", signature_path));
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "action": "sign_file",
                    "status": "error",
                    "file": file_path,
                    "error": e.to_string()
                }));
            } else {
                output_text(&format!("❌ Failed to sign file: {}", file_path));
                output_text(&format!("Error: {}", e));
            }
            return Err(anyhow!("Failed to sign file: {}", e));
        }
    }

    Ok(())
}

async fn verify_target(target: &str, json_output: bool) -> Result<()> {
    info!("Verifying target: {}", target);

    match target {
        "commit" => verify_commit_signatures(json_output).await,
        "tag" => verify_tag_signatures(json_output).await,
        _ => {
            // Assume it's a file path
            verify_file_signature(target, json_output).await
        }
    }
}

async fn verify_commit_signatures(json_output: bool) -> Result<()> {
    if !Path::new(".git").exists() {
        if json_output {
            output_json(&json!({
                "error": "Not in a git repository",
                "action": "verify_commits"
            }));
        } else {
            output_text("❌ Not in a git repository");
        }
        return Err(anyhow!("Not in a git repository"));
    }

    match run_command("git", &["log", "--show-signature", "-n", "5", "--oneline"]) {
        Ok(output) => {
            let log_output = String::from_utf8_lossy(&output.stdout);
            
            if json_output {
                output_json(&json!({
                    "action": "verify_commits",
                    "status": "success",
                    "output": log_output.trim()
                }));
            } else {
                output_text("🔍 Recent Commit Signatures");
                output_text("===========================");
                output_text(&log_output);
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "action": "verify_commits",
                    "status": "error",
                    "error": e.to_string()
                }));
            } else {
                output_text("❌ Failed to verify commit signatures");
                output_text(&format!("Error: {}", e));
            }
            return Err(anyhow!("Failed to verify commits: {}", e));
        }
    }

    Ok(())
}

async fn verify_tag_signatures(json_output: bool) -> Result<()> {
    if !Path::new(".git").exists() {
        if json_output {
            output_json(&json!({
                "error": "Not in a git repository",
                "action": "verify_tags"
            }));
        } else {
            output_text("❌ Not in a git repository");
        }
        return Err(anyhow!("Not in a git repository"));
    }

    match run_command("git", &["tag", "-v"]) {
        Ok(output) => {
            let tags_output = String::from_utf8_lossy(&output.stdout);
            
            if json_output {
                output_json(&json!({
                    "action": "verify_tags",
                    "status": "success",
                    "output": tags_output.trim()
                }));
            } else {
                output_text("🏷️  Tag Signature Verification");
                output_text("=============================");
                if tags_output.trim().is_empty() {
                    output_text("No signed tags found");
                } else {
                    output_text(&tags_output);
                }
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "action": "verify_tags",
                    "status": "error",
                    "error": e.to_string()
                }));
            } else {
                output_text("❌ Failed to verify tag signatures");
                output_text(&format!("Error: {}", e));
            }
            return Err(anyhow!("Failed to verify tags: {}", e));
        }
    }

    Ok(())
}

async fn verify_file_signature(file_path: &str, json_output: bool) -> Result<()> {
    if !Path::new(file_path).exists() {
        if json_output {
            output_json(&json!({
                "error": "File not found",
                "file": file_path,
                "action": "verify_file"
            }));
        } else {
            output_text(&format!("❌ File not found: {}", file_path));
        }
        return Err(anyhow!("File not found"));
    }

    let signature_path = format!("{}.sig", file_path);
    if !Path::new(&signature_path).exists() {
        if json_output {
            output_json(&json!({
                "error": "Signature file not found",
                "file": file_path,
                "expected_signature": signature_path,
                "action": "verify_file"
            }));
        } else {
            output_text(&format!("❌ Signature file not found: {}", signature_path));
        }
        return Err(anyhow!("Signature file not found"));
    }

    match run_command("gpg", &["--verify", &signature_path, file_path]) {
        Ok(output) => {
            let verify_output = String::from_utf8_lossy(&output.stderr); // GPG outputs to stderr
            
            if json_output {
                output_json(&json!({
                    "action": "verify_file",
                    "status": "success",
                    "file": file_path,
                    "signature": signature_path,
                    "verification_output": verify_output.trim()
                }));
            } else {
                output_text(&format!("✅ File signature verified: {}", file_path));
                output_text("🔍 Verification details:");
                output_text(&verify_output);
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "action": "verify_file",
                    "status": "error",
                    "file": file_path,
                    "error": e.to_string()
                }));
            } else {
                output_text(&format!("❌ Failed to verify file signature: {}", file_path));
                output_text(&format!("Error: {}", e));
            }
            return Err(anyhow!("Failed to verify file: {}", e));
        }
    }

    Ok(())
}

async fn setup_gpg_for_rust(json_output: bool) -> Result<()> {
    info!("Setting up GPG for Rust development...");

    let mut setup_steps = Vec::new();
    let mut has_errors = false;

    // Check if GPG is installed
    match run_command("gpg", &["--version"]) {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("unknown")
                .to_string();
            setup_steps.push(json!({
                "step": "check_gpg",
                "status": "success",
                "message": "GPG is installed",
                "details": version
            }));
        }
        Err(_) => {
            has_errors = true;
            setup_steps.push(json!({
                "step": "check_gpg",
                "status": "error",
                "message": "GPG is not installed",
                "suggestion": "Install GPG using your system package manager"
            }));
        }
    }

    // List existing GPG keys
    match run_command("gpg", &["--list-secret-keys", "--keyid-format", "LONG"]) {
        Ok(output) => {
            let keys_output = String::from_utf8_lossy(&output.stdout);
            if keys_output.trim().is_empty() {
                setup_steps.push(json!({
                    "step": "check_keys",
                    "status": "warning",
                    "message": "No GPG keys found",
                    "suggestion": "Generate a new GPG key for signing"
                }));
            } else {
                setup_steps.push(json!({
                    "step": "check_keys",
                    "status": "success",
                    "message": "GPG keys found",
                    "keys": keys_output.trim()
                }));
            }
        }
        Err(_) => {
            has_errors = true;
            setup_steps.push(json!({
                "step": "check_keys",
                "status": "error",
                "message": "Failed to list GPG keys"
            }));
        }
    }

    // Check git configuration
    let mut git_config_status = "unconfigured";
    if let Ok(output) = run_command("git", &["config", "user.signingkey"]) {
        let signing_key = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !signing_key.is_empty() {
            git_config_status = "configured";
            setup_steps.push(json!({
                "step": "check_git_config",
                "status": "success",
                "message": "Git signing key configured",
                "signing_key": signing_key
            }));
        }
    }

    if git_config_status == "unconfigured" {
        setup_steps.push(json!({
            "step": "check_git_config",
            "status": "warning",
            "message": "Git signing key not configured",
            "suggestion": "Configure with: git config --global user.signingkey <key-id>"
        }));
    }

    // Check commit signing setting
    let commit_signing = run_command("git", &["config", "commit.gpgsign"])
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_default();

    if commit_signing == "true" {
        setup_steps.push(json!({
            "step": "check_commit_signing",
            "status": "success",
            "message": "Automatic commit signing enabled"
        }));
    } else {
        setup_steps.push(json!({
            "step": "check_commit_signing",
            "status": "info",
            "message": "Automatic commit signing disabled",
            "suggestion": "Enable with: git config --global commit.gpgsign true"
        }));
    }

    if json_output {
        output_json(&json!({
            "action": "setup_gpg",
            "status": if has_errors { "error" } else { "success" },
            "setup_steps": setup_steps
        }));
    } else {
        output_text("🔑 GPG Setup for Rust Development");
        output_text("=================================");
        
        for step in &setup_steps {
            let status = step["status"].as_str().unwrap_or("unknown");
            let message = step["message"].as_str().unwrap_or("Unknown");
            let icon = match status {
                "success" => "✅",
                "warning" => "⚠️ ",
                "error" => "❌",
                "info" => "ℹ️ ",
                _ => "❓",
            };
            
            output_text(&format!("{} {}", icon, message));
            
            if let Some(suggestion) = step.get("suggestion").and_then(|s| s.as_str()) {
                output_text(&format!("  💡 {}", suggestion));
            }
            
            if let Some(details) = step.get("details").and_then(|d| d.as_str()) {
                output_text(&format!("  📋 {}", details));
            }
        }
        
        if !has_errors {
            output_text("");
            output_text("🎉 GPG setup looks good!");
            output_text("");
            output_text("💡 Quick commands:");
            output_text("   • Sign last commit: oxy gpg sign commit");
            output_text("   • Create signed tag: git tag -s v1.0.0 -m 'Version 1.0.0'");
            output_text("   • Verify signatures: oxy gpg verify commit");
        }
    }

    Ok(())
}