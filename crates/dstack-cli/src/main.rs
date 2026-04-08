mod cmd_audit;
mod cmd_deploy;
mod cmd_memory;
mod cmd_sync;
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
    /// Deploy a service (build + restart + smoke test)
    Deploy {
        /// Service name from config, or --all
        #[arg(default_value = "")]
        service: String,
        /// Deploy all configured services
        #[arg(long)]
        all: bool,
        /// Rollback to previous binary
        #[arg(long)]
        rollback: bool,
    },
    /// Git sync across tracked repos
    Sync {
        /// Show status without syncing
        #[arg(long)]
        status: bool,
        /// Dry run (don't push)
        #[arg(long)]
        dry_run: bool,
    },
    /// Quality audit
    Audit {
        /// Run pre-commit quality gate
        #[arg(long)]
        pre_commit: bool,
        /// Scan for stale companion docs
        #[arg(long)]
        stale: bool,
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
        Commands::Deploy { service, all, rollback } => {
            if rollback {
                if service.is_empty() {
                    anyhow::bail!("Specify a service name for rollback");
                }
                cmd_deploy::rollback(&cfg, &service)?;
            } else if all {
                cmd_deploy::deploy_all(&cfg)?;
            } else if service.is_empty() {
                anyhow::bail!("Specify a service name or use --all");
            } else {
                cmd_deploy::deploy(&cfg, &service)?;
            }
        }
        Commands::Sync { status, dry_run } => {
            if status {
                cmd_sync::status(&cfg)?;
            } else {
                cmd_sync::sync(&cfg, dry_run)?;
            }
        }
        Commands::Audit { pre_commit, stale } => {
            if pre_commit {
                cmd_audit::pre_commit()?;
            } else if stale {
                cmd_audit::stale(&cfg)?;
            } else {
                cmd_audit::summary(&cfg)?;
            }
        }
    }

    Ok(())
}
