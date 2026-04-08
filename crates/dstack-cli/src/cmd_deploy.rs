use crate::config::Config;
use std::process::Command;

pub fn deploy(cfg: &Config, service: &str) -> anyhow::Result<()> {
    let target = cfg.deploy.get(service)
        .ok_or_else(|| anyhow::anyhow!("No deploy target '{}'. Available: {:?}", service, cfg.deploy.keys().collect::<Vec<_>>()))?;

    eprintln!("=== Deploying {} ===", service);

    // 1. Build
    eprintln!("[1/3] Building...");
    run_cmd(&target.build, "Build")?;

    // 2. Restart service
    eprintln!("[2/3] Restarting {}...", target.service);
    run_cmd(&format!("sudo systemctl restart {}", target.service), "Restart")?;

    // 3. Smoke test
    if let Some(ref smoke) = target.smoke {
        eprintln!("[3/3] Smoke test...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        match run_cmd(smoke, "Smoke test") {
            Ok(_) => eprintln!("Smoke test passed."),
            Err(e) => eprintln!("WARNING: Smoke test failed: {}. Consider rollback.", e),
        }
    } else {
        eprintln!("[3/3] No smoke test configured, skipping.");
    }

    eprintln!("=== {} deployed ===", service);
    Ok(())
}

pub fn deploy_all(cfg: &Config) -> anyhow::Result<()> {
    if cfg.deploy.is_empty() {
        anyhow::bail!("No deploy targets configured in config.toml");
    }
    for name in cfg.deploy.keys() {
        deploy(cfg, name)?;
    }
    Ok(())
}

fn run_cmd(cmd: &str, label: &str) -> anyhow::Result<()> {
    let status = Command::new("bash").arg("-c").arg(cmd).status()?;
    if !status.success() {
        anyhow::bail!("{} failed (exit {})", label, status.code().unwrap_or(-1));
    }
    Ok(())
}
