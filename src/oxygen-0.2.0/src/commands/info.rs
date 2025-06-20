use crate::utils::{is_rust_project, output_json, output_text, run_command};
use anyhow::Result;
use serde_json::json;
use std::path::Path;
use tracing::info;

pub async fn run(json_output: bool) -> Result<()> {
    info!("Gathering project information...");

    let mut project_info = json!({});

    // Check if we're in a Rust project
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

    project_info["is_rust_project"] = json!(true);

    // Read Cargo.toml
    if let Ok(cargo_content) = std::fs::read_to_string("Cargo.toml") {
        if let Ok(manifest) = cargo_content.parse::<toml::Value>() {
            if let Some(package) = manifest.get("package") {
                project_info["package"] = json!({
                    "name": package.get("name").and_then(|v| v.as_str()),
                    "version": package.get("version").and_then(|v| v.as_str()),
                    "edition": package.get("edition").and_then(|v| v.as_str()),
                    "authors": package.get("authors"),
                    "description": package.get("description").and_then(|v| v.as_str()),
                });
            }

            if let Some(dependencies) = manifest.get("dependencies") {
                project_info["dependencies_count"] =
                    json!(dependencies.as_table().map(|t| t.len()).unwrap_or(0));
            }

            if let Some(dev_dependencies) = manifest.get("dev-dependencies") {
                project_info["dev_dependencies_count"] =
                    json!(dev_dependencies.as_table().map(|t| t.len()).unwrap_or(0));
            }
        }
    }

    // Git information
    if Path::new(".git").exists() {
        let mut git_info = json!({});

        // Get current branch
        if let Ok(output) = run_command("git", &["branch", "--show-current"]) {
            git_info["current_branch"] = json!(String::from_utf8_lossy(&output.stdout).trim());
        }

        // Get git status
        if let Ok(output) = run_command("git", &["status", "--porcelain"]) {
            let status_output = String::from_utf8_lossy(&output.stdout);
            let status_lines: Vec<&str> = status_output.lines().collect();
            git_info["dirty_files"] = json!(status_lines.len());
            git_info["is_clean"] = json!(status_lines.is_empty());
        }

        // Get last commit
        if let Ok(output) = run_command(
            "git",
            &["log", "-1", "--pretty=format:%H|%s|%an|%ad", "--date=short"],
        ) {
            let commit_info = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = commit_info.split('|').collect();
            if parts.len() >= 4 {
                git_info["last_commit"] = json!({
                    "hash": parts[0],
                    "message": parts[1],
                    "author": parts[2],
                    "date": parts[3]
                });
            }
        }

        project_info["git"] = git_info;
    } else {
        project_info["git"] = json!({ "is_git_repo": false });
    }

    // Check for common files
    let common_files = [
        "README.md",
        "LICENSE",
        "CHANGELOG.md",
        ".gitignore",
        "rust-toolchain.toml",
    ];
    let mut found_files = Vec::new();
    for file in &common_files {
        if Path::new(file).exists() {
            found_files.push(file);
        }
    }
    project_info["common_files"] = json!(found_files);

    // Check target directory size if it exists
    if let Ok(metadata) = std::fs::metadata("target") {
        if metadata.is_dir() {
            project_info["has_target_dir"] = json!(true);
            // Note: Getting exact size would require recursive directory traversal
            // For now, just note its existence
        } else {
            project_info["has_target_dir"] = json!(false);
        }
    } else {
        project_info["has_target_dir"] = json!(false);
    }

    if json_output {
        output_json(&project_info);
    } else {
        output_text("📦 Project Information");
        output_text("======================");

        if let Some(package) = project_info["package"].as_object() {
            if let Some(name) = package["name"].as_str() {
                output_text(&format!("Name: {}", name));
            }
            if let Some(version) = package["version"].as_str() {
                output_text(&format!("Version: {}", version));
            }
            if let Some(edition) = package["edition"].as_str() {
                output_text(&format!("Edition: {}", edition));
            }
            if let Some(desc) = package["description"].as_str() {
                output_text(&format!("Description: {}", desc));
            }
        }

        if let Some(deps) = project_info["dependencies_count"].as_u64() {
            output_text(&format!("Dependencies: {}", deps));
        }

        if let Some(dev_deps) = project_info["dev_dependencies_count"].as_u64() {
            output_text(&format!("Dev Dependencies: {}", dev_deps));
        }

        output_text("");

        if let Some(git) = project_info["git"].as_object() {
            if git
                .get("is_git_repo")
                .and_then(|v| v.as_bool())
                .unwrap_or(true)
            {
                output_text("📝 Git Status:");
                if let Some(branch) = git["current_branch"].as_str() {
                    output_text(&format!("  Branch: {}", branch));
                }
                if let Some(is_clean) = git["is_clean"].as_bool() {
                    let status = if is_clean {
                        "Clean"
                    } else {
                        "Modified files present"
                    };
                    output_text(&format!("  Status: {}", status));
                }
                if let Some(commit) = git["last_commit"].as_object() {
                    if let (Some(msg), Some(author), Some(date)) = (
                        commit["message"].as_str(),
                        commit["author"].as_str(),
                        commit["date"].as_str(),
                    ) {
                        output_text(&format!("  Last Commit: {} by {} ({})", msg, author, date));
                    }
                }
                output_text("");
            } else {
                output_text("📝 Git: Not a git repository");
                output_text("");
            }
        }

        if let Some(files) = project_info["common_files"].as_array() {
            if !files.is_empty() {
                output_text("📄 Project Files:");
                for file in files {
                    if let Some(filename) = file.as_str() {
                        output_text(&format!("  ✅ {}", filename));
                    }
                }
            }
        }

        if let Some(has_target) = project_info["has_target_dir"].as_bool() {
            if has_target {
                output_text("  📁 target/ directory exists");
            }
        }
    }

    Ok(())
}
