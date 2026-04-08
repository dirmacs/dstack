use crate::config::Config;
use std::process::Command;

pub fn status(cfg: &Config) -> anyhow::Result<()> {
    if cfg.repos.tracked.is_empty() {
        eprintln!("No repos tracked. Add repos to [repos] in config.toml");
        return Ok(());
    }
    println!("{:<20} {:<10} {}", "REPO", "BRANCH", "STATUS");
    println!("{}", "-".repeat(50));
    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if !std::path::Path::new(&path).exists() {
            println!("{:<20} {:<10} {}", repo, "-", "NOT FOUND");
            continue;
        }
        let branch = git_output(&path, &["branch", "--show-current"]);
        let dirty = git_output(&path, &["status", "--porcelain"]);
        let dirty_count = dirty.lines().filter(|l| !l.is_empty()).count();
        let status_str = if dirty_count > 0 {
            format!("{} dirty file(s)", dirty_count)
        } else {
            "clean".into()
        };
        println!("{:<20} {:<10} {}", repo, branch.trim(), status_str);
    }
    Ok(())
}

pub fn sync(cfg: &Config, dry_run: bool) -> anyhow::Result<()> {
    if cfg.repos.tracked.is_empty() {
        anyhow::bail!("No repos tracked. Add repos to [repos] in config.toml");
    }
    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if !std::path::Path::new(&path).exists() {
            eprintln!("{}: NOT FOUND, skipping", repo);
            continue;
        }
        let dirty = git_output(&path, &["status", "--porcelain"]);
        if !dirty.trim().is_empty() {
            eprintln!("{}: dirty — skipping (commit first)", repo);
            continue;
        }
        if dry_run {
            eprintln!("{}: clean (dry-run, would pull+push)", repo);
            continue;
        }
        eprint!("{}: ", repo);
        let pull = Command::new("git").args(["-C", &path, "pull", "--ff-only"]).status()?;
        if pull.success() {
            let push = Command::new("git").args(["-C", &path, "push"]).status()?;
            eprintln!("{}", if push.success() { "synced" } else { "push failed" });
        } else {
            eprintln!("pull failed (diverged?)");
        }
    }
    Ok(())
}

fn git_output(repo_path: &str, args: &[&str]) -> String {
    Command::new("git")
        .args([&["-C", repo_path], args].concat())
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}
