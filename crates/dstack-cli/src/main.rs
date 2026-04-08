mod cmd_memory;
mod config;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dstack", version, about = "Development stack for AI-assisted multi-repo work")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show current configuration
    Config,
    /// Memory operations (load, save, query, export)
    Memory {
        #[command(subcommand)]
        action: MemoryAction,
    },
}

#[derive(Subcommand)]
enum MemoryAction {
    /// Load memory fields for a project
    Load {
        #[arg(short, long)]
        project: Option<String>,
    },
    /// Save a key-value pair to memory
    Save {
        /// Field path (e.g. "projects/myapp/learnings/auth-fix")
        key: String,
        /// Field value
        value: String,
    },
    /// Search memory by keyword
    Query {
        /// Search pattern
        pattern: String,
    },
    /// Export all memory as JSON
    Export,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = config::Config::load()?;

    match cli.command {
        Commands::Config => {
            println!("Config path: {}", config::config_path().display());
            println!("Memory backend: {}", cfg.memory.backend);
            println!("Memory path: {}", cfg.memory_path().display());
            println!("Tracked repos: {:?}", cfg.repos.tracked);
            println!("Deploy targets: {:?}", cfg.deploy.keys().collect::<Vec<_>>());
        }
        Commands::Memory { action } => match action {
            MemoryAction::Load { project } => {
                cmd_memory::load(&cfg, project.as_deref()).await?;
            }
            MemoryAction::Save { key, value } => {
                cmd_memory::save(&cfg, &key, &value).await?;
            }
            MemoryAction::Query { pattern } => {
                cmd_memory::query(&cfg, &pattern).await?;
            }
            MemoryAction::Export => {
                cmd_memory::export(&cfg).await?;
            }
        },
    }

    Ok(())
}
