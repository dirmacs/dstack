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
        }
    }
    Ok(())
}
