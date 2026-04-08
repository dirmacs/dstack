use crate::config::Config;

pub fn pre_commit() -> anyhow::Result<()> {
    println!("=== dstack quality gate ===\n");
    let questions = [
        ("Negative tests?", "Did I test what happens when things go WRONG? (wrong password → 403, invalid input → error, missing field → handled)"),
        ("Live verification?", "Did I verify against the LIVE system? (not just compilation — actual curl, DB query, or Chrome DevTools)"),
        ("Companion doc updated?", "Did I update the .implementation.md with DETAILS? (bugs found, DB state, commit hashes — not one-liner 'DONE')"),
        ("Tests prove the change?", "Would these tests FAIL without my code change? (decorative tests that pass regardless are worthless)"),
        ("Truly done?", "Am I moving forward because it's TRULY done, or because I want to show progress?"),
    ];
    for (i, (short, detail)) in questions.iter().enumerate() {
        println!("  {}. {} — {}", i + 1, short, detail);
    }
    println!("\nAnswer honestly before committing. These exist because skipping them cost us a client meeting.");
    Ok(())
}

pub fn stale(cfg: &Config) -> anyhow::Result<()> {
    eprintln!("Scanning for stale companion docs across tracked repos...\n");
    let mut found = 0;
    let now = std::time::SystemTime::now();

    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if !std::path::Path::new(&path).exists() {
            continue;
        }
        // Find .implementation.md files
        let output = std::process::Command::new("find")
            .args([&path, "-name", "*.implementation.md", "-type", "f"])
            .output()?;
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.is_empty() {
                continue;
            }
            let file_path = std::path::Path::new(line);
            if let Ok(meta) = file_path.metadata() {
                if let Ok(modified) = meta.modified() {
                    if let Ok(age) = now.duration_since(modified) {
                        let days = age.as_secs() / 86400;
                        if days > 14 {
                            println!("STALE ({:>3}d): {}", days, line);
                            found += 1;
                        } else if days > 7 {
                            println!("AGING ({:>3}d): {}", days, line);
                        }
                    }
                }
            }
        }
    }
    if found == 0 {
        println!("No stale companion docs found (>14 days).");
    } else {
        println!("\n{} stale file(s). Update or remove them.", found);
    }
    Ok(())
}

pub fn summary(cfg: &Config) -> anyhow::Result<()> {
    println!("=== dstack audit summary ===\n");

    // Count tracked repos
    let total_repos = cfg.repos.tracked.len();
    let existing: Vec<_> = cfg
        .repos
        .tracked
        .iter()
        .filter(|r| {
            std::path::Path::new(&format!("{}/{}", cfg.repos.root, r)).exists()
        })
        .collect();

    println!("Repos: {}/{} found", existing.len(), total_repos);
    println!("Deploy targets: {}", cfg.deploy.len());
    println!("Memory backend: {}", cfg.memory.backend);

    // Check for dirty repos
    let mut dirty = 0;
    for repo in &existing {
        let path = format!("{}/{}", cfg.repos.root, repo);
        let output = std::process::Command::new("git")
            .args(["-C", &path, "status", "--porcelain"])
            .output()?;
        let out = String::from_utf8_lossy(&output.stdout);
        if !out.trim().is_empty() {
            dirty += 1;
        }
    }
    println!("Dirty repos: {}/{}", dirty, existing.len());

    println!("\nQuality gate: dstack audit --pre-commit");
    println!("Stale docs:   dstack audit --stale");
    Ok(())
}
