use crate::config::Config;
use std::process::Command;

pub fn deploy(cfg: &Config, service: &str) -> anyhow::Result<()> {
    let target = cfg
        .deploy
        .get(service)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No deploy target '{}'. Available: {:?}",
                service,
                cfg.deploy.keys().collect::<Vec<_>>()
            )
        })?;

    eprintln!("=== Deploying {} ===", service);

    // 0. Disk check
    eprintln!("[0/4] Checking disk space...");
    let df = cmd_output("df -h /opt | tail -1")?;
    let use_pct = parse_disk_usage(&df);
    if use_pct >= 90 {
        anyhow::bail!(
            "Disk usage at {}%. Aborting deploy — clean target dirs first.\n  {}",
            use_pct,
            df.trim()
        );
    }
    eprintln!("  Disk: {}% used", use_pct);

    // 1. Backup current binary
    let binary_name = &target.service;
    let backup_path = format!("/tmp/dstack-rollback-{}", binary_name);
    let binary_search = cmd_output(&format!(
        "which {} 2>/dev/null || find /opt/{} -name {} -path '*/release/*' 2>/dev/null | head -1",
        binary_name, service, binary_name
    ))?;
    let binary_path = binary_search.trim().lines().next().unwrap_or("");
    if !binary_path.is_empty() && std::path::Path::new(binary_path).exists() {
        eprintln!("[1/4] Backing up {} → {}", binary_path, backup_path);
        let _ = std::fs::copy(binary_path, &backup_path);
    } else {
        eprintln!("[1/4] No existing binary found, skipping backup");
    }

    // 2. Build
    eprintln!("[2/4] Building...");
    run_cmd(&target.build, "Build")?;

    // 3. Restart service
    eprintln!("[3/4] Restarting {}...", target.service);
    run_cmd(
        &format!("sudo systemctl restart {}", target.service),
        "Restart",
    )?;

    // 4. Smoke test
    if let Some(ref smoke) = target.smoke {
        eprintln!("[4/4] Smoke test...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        match run_cmd(smoke, "Smoke test") {
            Ok(_) => eprintln!("  Smoke test passed."),
            Err(e) => {
                eprintln!("  WARNING: Smoke test failed: {}", e);
                if std::path::Path::new(&backup_path).exists() {
                    eprintln!(
                        "  Rollback available: dstack deploy {} --rollback",
                        service
                    );
                }
            }
        }
    } else {
        eprintln!("[4/4] No smoke test configured, skipping.");
    }

    eprintln!("=== {} deployed ===", service);
    Ok(())
}

pub fn rollback(cfg: &Config, service: &str) -> anyhow::Result<()> {
    let target = cfg
        .deploy
        .get(service)
        .ok_or_else(|| anyhow::anyhow!("No deploy target '{}'", service))?;

    let backup_path = format!("/tmp/dstack-rollback-{}", target.service);
    if !std::path::Path::new(&backup_path).exists() {
        anyhow::bail!("No rollback binary found at {}", backup_path);
    }

    // Find where the current binary lives
    let binary_search = cmd_output(&format!(
        "find /opt/{} -name {} -path '*/release/*' 2>/dev/null | head -1",
        service, target.service
    ))?;
    let binary_path = binary_search.trim();
    if binary_path.is_empty() {
        anyhow::bail!("Cannot find current binary path for {}", service);
    }

    eprintln!("=== Rolling back {} ===", service);
    eprintln!("[1/2] Restoring {} → {}", backup_path, binary_path);
    std::fs::copy(&backup_path, binary_path)?;

    eprintln!("[2/2] Restarting {}...", target.service);
    run_cmd(
        &format!("sudo systemctl restart {}", target.service),
        "Restart",
    )?;

    // Smoke test the rollback
    if let Some(ref smoke) = target.smoke {
        std::thread::sleep(std::time::Duration::from_secs(2));
        match run_cmd(smoke, "Smoke test") {
            Ok(_) => eprintln!("  Rollback smoke test passed."),
            Err(e) => eprintln!("  WARNING: Rollback smoke test failed: {}", e),
        }
    }

    eprintln!("=== {} rolled back ===", service);
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

fn cmd_output(cmd: &str) -> anyhow::Result<String> {
    let output = Command::new("bash").arg("-c").arg(cmd).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_disk_usage(df_line: &str) -> u32 {
    // Parse "Use%" column from df output (e.g., "45%")
    df_line
        .split_whitespace()
        .find(|s| s.ends_with('%'))
        .and_then(|s| s.trim_end_matches('%').parse().ok())
        .unwrap_or(0)
}
