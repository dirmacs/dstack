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
    eprintln!("Scanning for stale files across tracked repos...\n");
    let mut found = 0;
    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if !std::path::Path::new(&path).exists() {
            continue;
        }
        // Find .implementation.md files older than 14 days
        let output = std::process::Command::new("find")
            .args([&path, "-name", "*.implementation.md", "-mtime", "+14", "-type", "f"])
            .output()?;
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if !line.is_empty() {
                println!("STALE (>14d): {}", line);
                found += 1;
            }
        }
    }
    if found == 0 {
        println!("No stale companion docs found.");
    } else {
        println!("\n{} stale file(s). Update or remove them.", found);
    }
    Ok(())
}
