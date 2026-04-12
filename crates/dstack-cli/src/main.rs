mod cmd_audit;
mod cmd_deploy;
mod cmd_init;
mod cmd_loop;
mod cmd_memory;
mod cmd_skills;
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
    /// Manage skills from dirmacs/skills repo
    Skills {
        #[command(subcommand)]
        action: SkillsAction,
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
    /// Manage long-running agent loops (ralph-style perpetual iteration)
    Loop {
        #[command(subcommand)]
        action: LoopAction,
    },
    /// Initialize a new plugin with all platform configs (Claude Code, Cursor, Pawan, Codex, OpenCode, Gemini)
    Init {
        /// Output directory (default: ./plugin)
        #[arg(default_value = "./plugin")]
        dir: String,
        /// Plugin name
        #[arg(short, long, default_value = "my-plugin")]
        name: String,
        /// Plugin description
        #[arg(short, long, default_value = "AI-assisted development plugin")]
        description: String,
        /// Author name
        #[arg(short, long, default_value = "author")]
        author: String,
        /// Show what would be generated without writing
        #[arg(long)]
        dry_run: bool,
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

#[derive(Subcommand)]
enum LoopAction {
    /// Create a new long-running loop
    Create {
        /// Loop name (alphanumeric, '-', '_'; max 64 chars)
        #[arg(short, long)]
        name: String,
        /// Cron expression (5 or 6 fields)
        #[arg(short, long)]
        cron: String,
        /// Task prompt the loop will execute each tick
        #[arg(short, long)]
        task: String,
        /// Print the request body without posting
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
enum SkillsAction {
    /// List available skills from dirmacs/skills repo
    List,
    /// Install a skill to ~/.claude/skills/
    Install {
        /// Skill name (e.g. "eruka-query")
        name: String,
    },
    /// Install all skills from the repo
    Sync,
    /// Pull latest and update installed skills
    Update,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = config::Config::load()?;
    cfg.load_env();

    match cli.command {
        Commands::Config => {
            println!("Config path: {}", config::config_path().display());
            println!("Env file: {}", cfg.env_file);
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
        Commands::Skills { action } => match action {
            SkillsAction::List => {
                cmd_skills::list(&cfg)?;
            }
            SkillsAction::Install { name } => {
                cmd_skills::install(&cfg, &name)?;
            }
            SkillsAction::Sync => {
                cmd_skills::sync_all(&cfg)?;
            }
            SkillsAction::Update => {
                cmd_skills::update(&cfg)?;
            }
        },
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
        Commands::Loop { action } => match action {
            LoopAction::Create { name, cron, task, dry_run } => {
                cmd_loop::create(&cfg, &name, &cron, &task, dry_run)?;
            }
        },
        Commands::Init { dir, name, description, author, dry_run } => {
            if dry_run {
                cmd_init::init_dry_run(&dir, &name);
            } else {
                cmd_init::init_plugin(&dir, &name, &description, &author)?;
            }
        }
    }

    Ok(())
}
