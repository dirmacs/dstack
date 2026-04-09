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

    match target.deploy_type.as_str() {
        "docker-compose" => deploy_docker_compose(service, target),
        _ => deploy_systemd(cfg, service, target),
    }
}

fn deploy_docker_compose(service: &str, target: &crate::config::DeployTarget) -> anyhow::Result<()> {
    let compose_file = target.compose_file.as_deref()
        .ok_or_else(|| anyhow::anyhow!("docker-compose deploy requires compose_file path"))?;

    eprintln!("=== Deploying {} (docker-compose) ===", service);

    // 1. Pull latest images
    eprintln!("[1/3] Pulling images...");
    run_cmd(
        &format!("docker compose -f {} pull {}", compose_file, target.service),
        "Docker pull",
    )?;

    // 2. Recreate containers
    eprintln!("[2/3] Restarting containers...");
    run_cmd(
        &format!("docker compose -f {} up -d {}", compose_file, target.service),
        "Docker up",
    )?;

    // 3. Smoke test
    if let Some(ref smoke) = target.smoke {
        eprintln!("[3/3] Smoke test...");
        std::thread::sleep(std::time::Duration::from_secs(5));
        match run_cmd(smoke, "Smoke test") {
            Ok(_) => eprintln!("  Smoke test passed."),
            Err(e) => eprintln!("  WARNING: Smoke test failed: {}", e),
        }
    } else {
        eprintln!("[3/3] No smoke test configured, skipping.");
    }

    eprintln!("=== {} deployed (docker-compose) ===", service);
    Ok(())
}

fn deploy_systemd(cfg: &Config, service: &str, target: &crate::config::DeployTarget) -> anyhow::Result<()> {
    eprintln!("=== Deploying {} ===", service);

    // 0. Disk check
    eprintln!("[0/4] Checking disk space...");
    let df = cmd_output("df -h / | tail -1")?;
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
        "which {} 2>/dev/null || find {}/{} -name {} -path '*/release/*' 2>/dev/null | head -1",
        binary_name, cfg.repos.root, service, binary_name
    ))?;
    let binary_path = binary_search.trim().lines().next().unwrap_or("");
    if !binary_path.is_empty() && std::path::Path::new(binary_path).exists() {
        eprintln!("[1/4] Backing up {} → {}", binary_path, backup_path);
        let _ = std::fs::copy(binary_path, &backup_path);
    } else {
        eprintln!("[1/4] No existing binary found, skipping backup");
    }

    // 2. Build
    if !target.build.is_empty() {
        eprintln!("[2/4] Building...");
        run_cmd(&target.build, "Build")?;
    } else {
        eprintln!("[2/4] No build command, skipping.");
    }

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
        "find {}/{} -name {} -path '*/release/*' 2>/dev/null | head -1",
        cfg.repos.root, service, target.service
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_config(deploy: HashMap<String, crate::config::DeployTarget>) -> Config {
        Config {
            deploy,
            ..Config::default()
        }
    }

    fn make_target(service: &str) -> crate::config::DeployTarget {
        crate::config::DeployTarget {
            deploy_type: "systemd".to_string(),
            build: String::new(),
            service: service.to_string(),
            compose_file: None,
            smoke: None,
        }
    }

    fn make_docker_target(service: &str, compose: Option<&str>) -> crate::config::DeployTarget {
        crate::config::DeployTarget {
            deploy_type: "docker-compose".to_string(),
            build: String::new(),
            service: service.to_string(),
            compose_file: compose.map(|s| s.to_string()),
            smoke: None,
        }
    }

    // --- parse_disk_usage tests ---

    #[test]
    fn test_parse_disk_usage_typical() {
        // Typical df output line
        let line = "/dev/sda1       50G   23G   25G  48% /";
        assert_eq!(parse_disk_usage(line), 48);
    }

    #[test]
    fn test_parse_disk_usage_high() {
        let line = "/dev/vda1       50G   46G   2G  92% /";
        assert_eq!(parse_disk_usage(line), 92);
    }

    #[test]
    fn test_parse_disk_usage_zero() {
        let line = "tmpfs           1G     0   1G   0% /tmp";
        assert_eq!(parse_disk_usage(line), 0);
    }

    #[test]
    fn test_parse_disk_usage_100() {
        let line = "/dev/sda1       50G   50G   0G 100% /";
        assert_eq!(parse_disk_usage(line), 100);
    }

    #[test]
    fn test_parse_disk_usage_empty() {
        assert_eq!(parse_disk_usage(""), 0);
    }

    #[test]
    fn test_parse_disk_usage_no_percent() {
        let line = "no percent sign here";
        assert_eq!(parse_disk_usage(line), 0);
    }

    #[test]
    fn test_parse_disk_usage_non_numeric_percent() {
        // find() returns first match ending with %; "abc%" can't parse → 0
        let line = "abc% def%";
        assert_eq!(parse_disk_usage(line), 0);
    }

    #[test]
    fn test_parse_disk_usage_mixed_tokens() {
        // First token with % is "45%" → parses to 45
        let line = "something 45% mounted";
        assert_eq!(parse_disk_usage(line), 45);
    }

    // --- deploy error path tests ---

    #[test]
    fn test_deploy_missing_service() {
        let cfg = test_config(HashMap::new());
        let err = deploy(&cfg, "nonexistent").unwrap_err();
        assert!(err.to_string().contains("No deploy target 'nonexistent'"));
    }

    #[test]
    fn test_deploy_all_empty_config() {
        let cfg = test_config(HashMap::new());
        let err = deploy_all(&cfg).unwrap_err();
        assert!(err.to_string().contains("No deploy targets configured"));
    }

    #[test]
    fn test_docker_compose_missing_compose_file() {
        let target = make_docker_target("myservice", None);
        let err = deploy_docker_compose("test-svc", &target).unwrap_err();
        assert!(err.to_string().contains("compose_file"));
    }

    #[test]
    fn test_rollback_missing_service() {
        let cfg = test_config(HashMap::new());
        let err = rollback(&cfg, "ghost").unwrap_err();
        assert!(err.to_string().contains("No deploy target 'ghost'"));
    }

    #[test]
    fn test_rollback_no_backup_file() {
        let mut map = HashMap::new();
        map.insert("test-svc".to_string(), make_target("test-svc-binary"));
        let cfg = test_config(map);
        let err = rollback(&cfg, "test-svc").unwrap_err();
        assert!(err.to_string().contains("No rollback binary found"));
    }

    #[test]
    fn test_deploy_routes_to_docker_compose() {
        let mut map = HashMap::new();
        map.insert("dc-svc".to_string(), make_docker_target("dc-svc", None));
        let cfg = test_config(map);
        // Should fail with compose_file error, proving it routed correctly
        let err = deploy(&cfg, "dc-svc").unwrap_err();
        assert!(err.to_string().contains("compose_file"));
    }

    #[test]
    fn test_deploy_target_defaults_to_systemd() {
        let target = make_target("my-service");
        assert_eq!(target.deploy_type, "systemd");
        assert!(target.compose_file.is_none());
        assert!(target.smoke.is_none());
    }
}
