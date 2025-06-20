use crate::utils::{output_json, output_text, run_command};
use crate::ToolchainAction;
use anyhow::{Result, anyhow};
use serde_json::json;
use tracing::info;

pub async fn run(action: ToolchainAction, json_output: bool) -> Result<()> {
    match action {
        ToolchainAction::List => list_toolchains(json_output).await,
        ToolchainAction::Install { toolchain } => install_toolchain(&toolchain, json_output).await,
        ToolchainAction::Default { toolchain } => set_default_toolchain(&toolchain, json_output).await,
        ToolchainAction::Show => show_active_toolchain(json_output).await,
        ToolchainAction::Remove { toolchain } => remove_toolchain(&toolchain, json_output).await,
    }
}

async fn list_toolchains(json_output: bool) -> Result<()> {
    info!("Listing installed toolchains...");

    let output = run_command("rustup", &["toolchain", "list"])?;
    let toolchain_output = String::from_utf8_lossy(&output.stdout);
    
    let mut toolchains = Vec::new();
    let mut default_toolchain = None;
    
    for line in toolchain_output.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        let (name, is_default) = if line.contains("(default)") {
            let name = line.replace("(default)", "").trim().to_string();
            default_toolchain = Some(name.clone());
            (name, true)
        } else {
            (line.to_string(), false)
        };
        
        toolchains.push(json!({
            "name": name,
            "is_default": is_default,
            "status": "installed"
        }));
    }

    if json_output {
        output_json(&json!({
            "toolchains": toolchains,
            "default": default_toolchain
        }));
    } else {
        output_text("🔧 Installed Rust Toolchains");
        output_text("============================");
        
        if toolchains.is_empty() {
            output_text("No toolchains installed");
        } else {
            for toolchain in &toolchains {
                let name = toolchain["name"].as_str().unwrap_or("unknown");
                let is_default = toolchain["is_default"].as_bool().unwrap_or(false);
                
                if is_default {
                    output_text(&format!("  {} (default) ✅", name));
                } else {
                    output_text(&format!("  {}", name));
                }
            }
        }
        
        output_text("");
        output_text("💡 Use `oxy toolchain install <name>` to install new toolchains");
        output_text("   Use `oxy toolchain default <name>` to set default toolchain");
    }

    Ok(())
}

async fn install_toolchain(toolchain: &str, json_output: bool) -> Result<()> {
    info!("Installing toolchain: {}", toolchain);

    if json_output {
        output_json(&json!({
            "action": "install",
            "toolchain": toolchain,
            "status": "starting"
        }));
    } else {
        output_text(&format!("📦 Installing toolchain: {}", toolchain));
    }

    match run_command("rustup", &["toolchain", "install", toolchain]) {
        Ok(output) => {
            let install_output = String::from_utf8_lossy(&output.stdout);
            
            if json_output {
                output_json(&json!({
                    "action": "install",
                    "toolchain": toolchain,
                    "status": "success",
                    "output": install_output.trim()
                }));
            } else {
                output_text(&format!("✅ Successfully installed toolchain: {}", toolchain));
                if !install_output.trim().is_empty() {
                    output_text(&format!("Output: {}", install_output.trim()));
                }
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "action": "install",
                    "toolchain": toolchain,
                    "status": "error",
                    "error": e.to_string()
                }));
            } else {
                output_text(&format!("❌ Failed to install toolchain: {}", toolchain));
                output_text(&format!("Error: {}", e));
            }
            return Err(anyhow!("Failed to install toolchain: {}", e));
        }
    }

    Ok(())
}

async fn set_default_toolchain(toolchain: &str, json_output: bool) -> Result<()> {
    info!("Setting default toolchain: {}", toolchain);

    match run_command("rustup", &["default", toolchain]) {
        Ok(_) => {
            if json_output {
                output_json(&json!({
                    "action": "set_default",
                    "toolchain": toolchain,
                    "status": "success"
                }));
            } else {
                output_text(&format!("✅ Set default toolchain to: {}", toolchain));
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "action": "set_default",
                    "toolchain": toolchain,
                    "status": "error",
                    "error": e.to_string()
                }));
            } else {
                output_text(&format!("❌ Failed to set default toolchain: {}", toolchain));
                output_text(&format!("Error: {}", e));
            }
            return Err(anyhow!("Failed to set default toolchain: {}", e));
        }
    }

    Ok(())
}

async fn show_active_toolchain(json_output: bool) -> Result<()> {
    info!("Showing active toolchain...");

    let output = run_command("rustup", &["show", "active-toolchain"])?;
    let active_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if json_output {
        output_json(&json!({
            "active_toolchain": active_output
        }));
    } else {
        output_text("🔧 Active Toolchain");
        output_text("==================");
        output_text(&format!("  {}", active_output));
    }

    Ok(())
}

async fn remove_toolchain(toolchain: &str, json_output: bool) -> Result<()> {
    info!("Removing toolchain: {}", toolchain);

    match run_command("rustup", &["toolchain", "uninstall", toolchain]) {
        Ok(_) => {
            if json_output {
                output_json(&json!({
                    "action": "remove",
                    "toolchain": toolchain,
                    "status": "success"
                }));
            } else {
                output_text(&format!("✅ Successfully removed toolchain: {}", toolchain));
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "action": "remove",
                    "toolchain": toolchain,
                    "status": "error",
                    "error": e.to_string()
                }));
            } else {
                output_text(&format!("❌ Failed to remove toolchain: {}", toolchain));
                output_text(&format!("Error: {}", e));
            }
            return Err(anyhow!("Failed to remove toolchain: {}", e));
        }
    }

    Ok(())
}