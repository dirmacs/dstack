mod api;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "dstack-serve", about = "dstack HTTP API server")]
struct Args {
    /// Port to bind to
    #[arg(long, default_value = "3500")]
    port: u16,

    /// Bind address
    #[arg(long, default_value = "127.0.0.1")]
    bind: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let args = Args::parse();
    let cfg = dstack::config::Config::load()?;
    cfg.load_env();

    let app = api::router(cfg);
    let addr = format!("{}:{}", args.bind, args.port);

    tracing::info!("dstack server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
