use crate::config::Config;
use std::process::Command;

pub fn status(cfg: &Config) -> anyhow::Result<()> {
    if cfg.repos.tracked.is_empty() {
        eprintln!("No repos tracked. Add repos to [repos] in config.toml");
        return Ok(());
    }
    // Fetch all remotes first for accurate ahead/behind
    eprint!("Fetching remotes...");
    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if std::path::Path::new(&path).exists() {
            let _ = Command::new("git")
                .args(["-C", &path, "fetch", "--quiet"])
                .status();
        }
    }
    eprintln!(" done.\n");

    println!(
        "{:<20} {:<10} {:<8} {:<8} {}",
        "REPO", "BRANCH", "AHEAD", "BEHIND", "STATUS"
    );
    println!("{}", "-".repeat(65));

    let mut dirty_repos = 0;
    let mut unpushed_repos = 0;

    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if !std::path::Path::new(&path).exists() {
            println!("{:<20} {:<10} {:<8} {:<8} NOT FOUND", repo, "-", "-", "-");
            continue;
        }
        let branch = git_output(&path, &["branch", "--show-current"]);
        let branch = branch.trim();

        // Ahead/behind tracking
        let ab_output = git_output(
            &path,
            &[
                "rev-list",
                "--left-right",
                "--count",
                &format!("{}...@{{u}}", branch),
            ],
        );
        let (ahead, behind) = parse_ahead_behind(&ab_output);

        // Dirty file count
        let dirty = git_output(&path, &["status", "--porcelain"]);
        let dirty_count = dirty.lines().filter(|l| !l.is_empty()).count();

        let mut status_parts = Vec::new();
        if dirty_count > 0 {
            status_parts.push(format!("{} dirty", dirty_count));
            dirty_repos += 1;
        }
        if ahead > 0 {
            status_parts.push(format!("{} to push", ahead));
            unpushed_repos += 1;
        }
        if behind > 0 {
            status_parts.push(format!("{} to pull", behind));
        }
        let status_str = if status_parts.is_empty() {
            "clean".to_string()
        } else {
            status_parts.join(", ")
        };

        let ahead_str = if ahead > 0 {
            format!("+{}", ahead)
        } else {
            "-".to_string()
        };
        let behind_str = if behind > 0 {
            format!("-{}", behind)
        } else {
            "-".to_string()
        };

        println!(
            "{:<20} {:<10} {:<8} {:<8} {}",
            repo, branch, ahead_str, behind_str, status_str
        );
    }

    // Summary
    println!("{}", "-".repeat(65));
    let total = cfg.repos.tracked.len();
    let clean = total - dirty_repos.max(unpushed_repos);
    println!(
        "{} repos: {} clean, {} dirty, {} unpushed",
        total, clean, dirty_repos, unpushed_repos
    );

    Ok(())
}

fn parse_ahead_behind(output: &str) -> (usize, usize) {
    let parts: Vec<&str> = output.trim().split('\t').collect();
    if parts.len() == 2 {
        let ahead = parts[0].parse().unwrap_or(0);
        let behind = parts[1].parse().unwrap_or(0);
        (ahead, behind)
    } else {
        (0, 0)
    }
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
