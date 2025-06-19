use crate::utils::{output_json, output_text, run_command};
use anyhow::{Result, anyhow};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::info;

pub async fn run(
    name: Option<String>, 
    template: Option<String>, 
    list_templates: bool, 
    json_output: bool
) -> Result<()> {
    if list_templates {
        return list_available_templates(json_output).await;
    }

    let project_name = match name {
        Some(n) => n,
        None => {
            if json_output {
                output_json(&json!({
                    "error": "Project name is required",
                    "usage": "oxy init <project_name> [--template <template>]"
                }));
            } else {
                output_text("❌ Project name is required");
                output_text("Usage: oxy init <project_name> [--template <template>]");
            }
            return Err(anyhow!("Project name is required"));
        }
    };

    let template_name = template.unwrap_or_else(|| "basic".to_string());
    
    initialize_project(&project_name, &template_name, json_output).await
}

async fn list_available_templates(json_output: bool) -> Result<()> {
    info!("Listing available project templates...");

    let templates = get_builtin_templates();

    if json_output {
        output_json(&json!({
            "templates": templates
        }));
    } else {
        output_text("📋 Available Project Templates");
        output_text("==============================");
        
        for (name, template) in &templates {
            output_text(&format!("🔹 {} - {}", name, template["description"].as_str().unwrap_or("No description")));
        }
        
        output_text("");
        output_text("💡 Usage: oxy init <project_name> --template <template_name>");
    }

    Ok(())
}

async fn initialize_project(project_name: &str, template_name: &str, json_output: bool) -> Result<()> {
    info!("Initializing project: {} with template: {}", project_name, template_name);

    if Path::new(project_name).exists() {
        if json_output {
            output_json(&json!({
                "error": "Directory already exists",
                "project_name": project_name
            }));
        } else {
            output_text(&format!("❌ Directory '{}' already exists", project_name));
        }
        return Err(anyhow!("Directory already exists"));
    }

    let templates = get_builtin_templates();
    let _template = match templates.get(template_name) {
        Some(t) => t,
        None => {
            if json_output {
                output_json(&json!({
                    "error": "Template not found",
                    "template": template_name,
                    "available_templates": templates.keys().collect::<Vec<_>>()
                }));
            } else {
                output_text(&format!("❌ Template '{}' not found", template_name));
                output_text("Available templates:");
                for name in templates.keys() {
                    output_text(&format!("  - {}", name));
                }
            }
            return Err(anyhow!("Template not found"));
        }
    };

    // Create the project directory
    fs::create_dir_all(project_name)?;

    match template_name {
        "basic" | "binary" => create_basic_project(project_name, json_output).await,
        "library" => create_library_project(project_name, json_output).await,
        "cli" => create_cli_project(project_name, json_output).await,
        "web-api" => create_web_api_project(project_name, json_output).await,
        "workspace" => create_workspace_project(project_name, json_output).await,
        _ => {
            if json_output {
                output_json(&json!({
                    "error": "Template implementation not found",
                    "template": template_name
                }));
            } else {
                output_text(&format!("❌ Template '{}' implementation not found", template_name));
            }
            Err(anyhow!("Template implementation not found"))
        }
    }
}

async fn create_basic_project(project_name: &str, json_output: bool) -> Result<()> {
    // Use cargo to create the basic structure
    match run_command("cargo", &["init", project_name, "--name", project_name]) {
        Ok(_) => {
            // Add some enhancements to the basic template
            let main_rs_content = r#"fn main() {
    println!("Hello, world! Welcome to {}!", env!("CARGO_PKG_NAME"));
    
    // Example: Reading command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("Arguments provided: {:?}", &args[1..]);
    }
}
"#;

            let cargo_toml_addition = r#"
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
opt-level = 0
debug = true
"#;

            let main_path = format!("{}/src/main.rs", project_name);
            let cargo_path = format!("{}/Cargo.toml", project_name);
            
            fs::write(&main_path, main_rs_content)?;
            
            // Append to Cargo.toml
            let mut cargo_content = fs::read_to_string(&cargo_path)?;
            cargo_content.push_str(cargo_toml_addition);
            fs::write(&cargo_path, cargo_content)?;

            // Create a basic README
            let readme_content = format!(r#"# {}

A Rust project created with Oxygen.

## Quick Start

```bash
cargo run
```

## Build for Release

```bash
cargo build --release
```

## Development

```bash
# Run with automatic recompilation
cargo watch -x run

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```
"#, project_name);

            fs::write(format!("{}/README.md", project_name), readme_content)?;

            if json_output {
                output_json(&json!({
                    "status": "success",
                    "project_name": project_name,
                    "template": "basic",
                    "files_created": ["src/main.rs", "Cargo.toml", "README.md"]
                }));
            } else {
                output_text(&format!("✅ Created basic Rust project: {}", project_name));
                output_text("📁 Project structure:");
                output_text(&format!("  {}/", project_name));
                output_text("    ├── src/main.rs");
                output_text("    ├── Cargo.toml");
                output_text("    └── README.md");
                output_text("");
                output_text(&format!("💡 Next steps: cd {} && cargo run", project_name));
            }
        }
        Err(e) => {
            if json_output {
                output_json(&json!({
                    "status": "error",
                    "error": e.to_string()
                }));
            } else {
                output_text(&format!("❌ Failed to create project: {}", e));
            }
            return Err(anyhow!("Failed to create project: {}", e));
        }
    }

    Ok(())
}

async fn create_library_project(project_name: &str, json_output: bool) -> Result<()> {
    match run_command("cargo", &["init", project_name, "--lib", "--name", project_name]) {
        Ok(_) => {
            let lib_rs_content = r#"//! # Project Name
//! 
//! A description of what this library does.

/// A sample function that adds two numbers.
/// 
/// # Examples
/// 
/// ```
/// use project_name::add;
/// 
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// A sample struct demonstrating library usage.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub version: String,
}

impl Config {
    /// Creates a new Config instance.
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_config() {
        let config = Config::new("test".to_string(), "1.0.0".to_string());
        assert_eq!(config.name, "test");
        assert_eq!(config.version, "1.0.0");
    }
}
"#;

            fs::write(format!("{}/src/lib.rs", project_name), lib_rs_content)?;

            if json_output {
                output_json(&json!({
                    "status": "success",
                    "project_name": project_name,
                    "template": "library",
                    "files_created": ["src/lib.rs", "Cargo.toml"]
                }));
            } else {
                output_text(&format!("✅ Created library project: {}", project_name));
                output_text("💡 Next steps:");
                output_text(&format!("  cd {} && cargo test", project_name));
                output_text("  cargo doc --open");
            }
        }
        Err(e) => return Err(anyhow!("Failed to create library project: {}", e))
    }

    Ok(())
}

async fn create_cli_project(project_name: &str, json_output: bool) -> Result<()> {
    match run_command("cargo", &["init", project_name, "--name", project_name]) {
        Ok(_) => {
            // Update Cargo.toml with CLI dependencies
            let cargo_toml_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = {{ version = "4.0", features = ["derive"] }}
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

[profile.release]
opt-level = 3
lto = true
strip = true
"#, project_name);

            let main_rs_content = r#"use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::fmt;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(about = "A CLI application built with Rust")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, help = "Enable verbose output")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello to someone
    Hello {
        /// Name of the person to greet
        #[arg(short, long, default_value = "World")]
        name: String,
    },
    /// Count something
    Count {
        /// Number to count to
        #[arg(short, long, default_value = "10")]
        number: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    fmt().with_max_level(level).init();

    info!("Starting CLI application");

    match cli.command {
        Commands::Hello { name } => {
            println!("Hello, {}! 👋", name);
            info!("Greeted {}", name);
        }
        Commands::Count { number } => {
            println!("Counting to {}:", number);
            for i in 1..=number {
                println!("  {}", i);
            }
            info!("Counted to {}", number);
        }
    }

    Ok(())
}
"#;

            fs::write(format!("{}/Cargo.toml", project_name), cargo_toml_content)?;
            fs::write(format!("{}/src/main.rs", project_name), main_rs_content)?;

            if json_output {
                output_json(&json!({
                    "status": "success",
                    "project_name": project_name,
                    "template": "cli",
                    "dependencies": ["clap", "anyhow", "tracing", "tracing-subscriber"]
                }));
            } else {
                output_text(&format!("✅ Created CLI project: {}", project_name));
                output_text("💡 Try: cargo run -- hello --name YourName");
            }
        }
        Err(e) => return Err(anyhow!("Failed to create CLI project: {}", e))
    }

    Ok(())
}

async fn create_web_api_project(project_name: &str, json_output: bool) -> Result<()> {
    match run_command("cargo", &["init", project_name, "--name", project_name]) {
        Ok(_) => {
            let cargo_toml_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = {{ version = "1.0", features = ["full"] }}
axum = "0.7"
tower = "0.4"
tower-http = {{ version = "0.5", features = ["cors", "trace"] }}
tracing = "0.1"
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
anyhow = "1.0"
"#, project_name);

            let main_rs_content = r#"use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    timestamp: u64,
}

#[derive(Deserialize)]
struct CreateItem {
    name: String,
    description: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/items", post(create_item))
        .route("/api/items/:id", get(get_item))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn root() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Welcome to the API!".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}

async fn health_check() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "OK".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}

async fn create_item(Json(payload): Json<CreateItem>) -> Result<Json<ApiResponse>, StatusCode> {
    info!("Creating item: {}", payload.name);
    
    Ok(Json(ApiResponse {
        message: format!("Created item: {}", payload.name),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }))
}

async fn get_item(Path(id): Path<String>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!("Item ID: {}", id),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}
"#;

            fs::write(format!("{}/Cargo.toml", project_name), cargo_toml_content)?;
            fs::write(format!("{}/src/main.rs", project_name), main_rs_content)?;

            if json_output {
                output_json(&json!({
                    "status": "success",
                    "project_name": project_name,
                    "template": "web-api",
                    "server_url": "http://localhost:3000"
                }));
            } else {
                output_text(&format!("✅ Created web API project: {}", project_name));
                output_text("💡 Start with: cargo run");
                output_text("   API will be available at http://localhost:3000");
            }
        }
        Err(e) => return Err(anyhow!("Failed to create web API project: {}", e))
    }

    Ok(())
}

async fn create_workspace_project(project_name: &str, json_output: bool) -> Result<()> {
    fs::create_dir_all(format!("{}/crates", project_name))?;

    let workspace_cargo_toml = format!(r#"[workspace]
members = [
    "crates/core",
    "crates/cli",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"

[workspace.dependencies]
anyhow = "1.0"
tokio = {{ version = "1.0", features = ["full"] }}
tracing = "0.1"
tracing-subscriber = "0.3"
serde = {{ version = "1.0", features = ["derive"] }}
clap = {{ version = "4.0", features = ["derive"] }}
"#);

    fs::write(format!("{}/Cargo.toml", project_name), workspace_cargo_toml)?;

    // Create core library
    run_command("cargo", &["init", &format!("{}/crates/core", project_name), "--lib", "--name", &format!("{}-core", project_name)])?;
    
    // Create CLI binary
    run_command("cargo", &["init", &format!("{}/crates/cli", project_name), "--name", &format!("{}-cli", project_name)])?;

    if json_output {
        output_json(&json!({
            "status": "success",
            "project_name": project_name,
            "template": "workspace",
            "crates": [format!("{}-core", project_name), format!("{}-cli", project_name)]
        }));
    } else {
        output_text(&format!("✅ Created workspace project: {}", project_name));
        output_text("📁 Workspace structure:");
        output_text(&format!("  {}/", project_name));
        output_text("    ├── Cargo.toml (workspace)");
        output_text("    └── crates/");
        output_text("        ├── core/ (library)");
        output_text("        └── cli/ (binary)");
        output_text("");
        output_text(&format!("💡 Build all: cd {} && cargo build", project_name));
    }

    Ok(())
}

fn get_builtin_templates() -> HashMap<String, serde_json::Value> {
    let mut templates = HashMap::new();
    
    templates.insert("basic".to_string(), json!({
        "description": "Basic Rust binary project with enhanced Cargo.toml",
        "type": "binary"
    }));
    
    templates.insert("binary".to_string(), json!({
        "description": "Alias for basic template",
        "type": "binary"
    }));
    
    templates.insert("library".to_string(), json!({
        "description": "Rust library with documentation and tests",
        "type": "library"
    }));
    
    templates.insert("cli".to_string(), json!({
        "description": "Command-line application with clap and tracing",
        "type": "binary",
        "dependencies": ["clap", "anyhow", "tracing"]
    }));
    
    templates.insert("web-api".to_string(), json!({
        "description": "Web API server using Axum framework",
        "type": "binary",
        "dependencies": ["axum", "tokio", "tower", "serde"]
    }));
    
    templates.insert("workspace".to_string(), json!({
        "description": "Multi-crate workspace with core library and CLI",
        "type": "workspace"
    }));
    
    templates
}