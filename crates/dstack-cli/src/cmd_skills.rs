use crate::config::Config;
use std::path::{Path, PathBuf};

fn skills_repo_path(cfg: &Config) -> anyhow::Result<PathBuf> {
    let path = cfg
        .skills_repo
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!(
            "No skills_repo configured. Set skills_repo in ~/.config/dstack/config.toml"
        ))?;
    let p = PathBuf::from(path);
    if !p.exists() {
        anyhow::bail!(
            "Skills repo not found at {}. Set skills_repo to a valid local path",
            path
        );
    }
    Ok(p)
}

fn claude_skills_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/root"))
        .join(".claude")
        .join("skills")
}

fn available_skills(repo: &Path) -> Vec<String> {
    let mut skills = Vec::new();
    if let Ok(entries) = std::fs::read_dir(repo) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("SKILL.md").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    skills.push(name.to_string());
                }
            }
        }
    }
    skills.sort();
    skills
}

pub fn list(cfg: &Config) -> anyhow::Result<()> {
    let repo = skills_repo_path(cfg)?;
    let target = claude_skills_dir();
    let skills = available_skills(&repo);

    if skills.is_empty() {
        eprintln!("No skills found in {}", repo.display());
        return Ok(());
    }

    println!(
        "{:<25} {:<10} {}",
        "SKILL", "STATUS", "DESCRIPTION"
    );
    println!("{}", "-".repeat(65));

    for name in &skills {
        let installed = target.join(name).join("SKILL.md").exists();
        let status = if installed { "installed" } else { "-" };

        // Read first line of description from SKILL.md frontmatter
        let desc = read_skill_description(&repo.join(name).join("SKILL.md"));

        println!("{:<25} {:<10} {}", name, status, desc);
    }

    println!(
        "\n{} skill(s) available. Install with: dstack skills install <name>",
        skills.len()
    );
    Ok(())
}

pub fn install(cfg: &Config, name: &str) -> anyhow::Result<()> {
    let repo = skills_repo_path(cfg)?;
    let source = repo.join(name).join("SKILL.md");
    if !source.exists() {
        anyhow::bail!(
            "Skill '{}' not found in {}. Run: dstack skills list",
            name,
            repo.display()
        );
    }

    let target_dir = claude_skills_dir().join(name);
    std::fs::create_dir_all(&target_dir)?;
    std::fs::copy(&source, target_dir.join("SKILL.md"))?;

    eprintln!("Installed: {} → {}", name, target_dir.display());
    Ok(())
}

pub fn sync_all(cfg: &Config) -> anyhow::Result<()> {
    let repo = skills_repo_path(cfg)?;
    let skills = available_skills(&repo);
    let mut installed = 0;
    let mut skipped = 0;

    for name in &skills {
        let target = claude_skills_dir().join(name).join("SKILL.md");
        if target.exists() {
            skipped += 1;
            continue;
        }
        install(cfg, name)?;
        installed += 1;
    }

    eprintln!(
        "\n{} installed, {} already present, {} total.",
        installed, skipped, skills.len()
    );
    Ok(())
}

pub fn update(cfg: &Config) -> anyhow::Result<()> {
    let repo = skills_repo_path(cfg)?;

    // Pull latest if it's a git repo
    if repo.join(".git").exists() {
        eprint!("Pulling latest skills... ");
        let status = std::process::Command::new("git")
            .args(["-C", &repo.to_string_lossy(), "pull", "--ff-only"])
            .status()?;
        if status.success() {
            eprintln!("done.");
        } else {
            eprintln!("pull failed (diverged?)");
        }
    }

    // Overwrite all installed skills with latest from repo
    let skills = available_skills(&repo);
    let target_root = claude_skills_dir();
    let mut updated = 0;

    for name in &skills {
        let target = target_root.join(name).join("SKILL.md");
        if target.exists() {
            let source = repo.join(name).join("SKILL.md");
            std::fs::copy(&source, &target)?;
            updated += 1;
        }
    }

    eprintln!("{} skill(s) updated from {}.", updated, repo.display());
    Ok(())
}

fn read_skill_description(path: &Path) -> String {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("description:") {
            return trimmed
                .trim_start_matches("description:")
                .trim()
                .to_string();
        }
    }
    String::new()
}
