//! Plugin scaffolding codegen — generates platform-specific config files
//! for Claude Code, Cursor, Pawan, Codex, OpenCode, and Gemini CLI.

use std::fs;
use std::path::Path;

/// Supported platforms for plugin scaffolding
pub const PLATFORMS: &[&str] = &[
    "claude-code",
    "cursor",
    "pawan",
    "codex",
    "opencode",
    "gemini",
];

/// Initialize a dstack plugin in the given directory with all platform configs.
pub fn init_plugin(dir: &str, name: &str, description: &str, author: &str) -> anyhow::Result<()> {
    let base = Path::new(dir);
    fs::create_dir_all(base)?;

    eprintln!("Initializing dstack plugin '{}' in {}", name, dir);

    // Core files
    write_package_json(base, name, description, author)?;
    write_claude_md(base, name)?;
    write_agents_md(base, name)?;
    write_changelog(base, name)?;

    // Platform manifests
    write_claude_plugin(base, name, description, author)?;
    write_cursor_plugin(base, name, description, author)?;
    write_pawan_plugin(base, name, description)?;
    write_gemini_extension(base, name, description)?;
    write_gemini_md(base)?;

    // Install docs
    write_codex_install(base, name)?;
    write_opencode_install(base, name)?;
    write_pawan_install(base, name)?;
    write_opencode_shim(base, name)?;

    // Hooks
    write_hooks(base)?;

    // Skeleton skill
    write_skeleton_skill(base, name)?;

    eprintln!("Created plugin scaffold with {} platform configs", PLATFORMS.len());
    eprintln!("\nGenerated:");
    eprintln!("  .claude-plugin/plugin.json    (Claude Code)");
    eprintln!("  .cursor-plugin/plugin.json    (Cursor)");
    eprintln!("  .pawan/plugin.toml            (Pawan)");
    eprintln!("  .codex/INSTALL.md             (Codex)");
    eprintln!("  .opencode/INSTALL.md          (OpenCode)");
    eprintln!("  gemini-extension.json         (Gemini CLI)");
    eprintln!("  hooks/hooks.json              (Claude Code hooks)");
    eprintln!("  hooks/hooks-cursor.json       (Cursor hooks)");
    eprintln!("  hooks/session-start           (session bootstrap)");
    eprintln!("  skills/using-{}/SKILL.md      (starter skill)", name);

    Ok(())
}

/// List what would be generated without writing.
pub fn init_dry_run(dir: &str, name: &str) {
    eprintln!("Would generate dstack plugin '{}' in {}:", name, dir);
    let files = [
        "package.json",
        "CLAUDE.md",
        "AGENTS.md",
        "GEMINI.md",
        "CHANGELOG.md",
        ".claude-plugin/plugin.json",
        ".cursor-plugin/plugin.json",
        ".pawan/plugin.toml",
        ".pawan/INSTALL.md",
        ".codex/INSTALL.md",
        ".opencode/INSTALL.md",
        ".opencode/plugins/{name}.js",
        "gemini-extension.json",
        "hooks/hooks.json",
        "hooks/hooks-cursor.json",
        "hooks/session-start",
        "skills/using-{name}/SKILL.md",
    ];
    for f in &files {
        eprintln!("  {}", f.replace("{name}", name));
    }
}

// === File generators ===

fn write_package_json(base: &Path, name: &str, desc: &str, author: &str) -> anyhow::Result<()> {
    let content = format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0",
  "description": "{}",
  "author": "{}",
  "license": "MIT",
  "homepage": "https://github.com/{}/{}",
  "repository": {{
    "type": "git",
    "url": "https://github.com/{}/{}"
  }},
  "keywords": ["dstack-plugin"]
}}
"#,
        name, desc, author, author, name, author, name
    );
    fs::write(base.join("package.json"), content)?;
    Ok(())
}

fn write_claude_plugin(base: &Path, name: &str, desc: &str, author: &str) -> anyhow::Result<()> {
    let dir = base.join(".claude-plugin");
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"{{
  "name": "{}",
  "description": "{}",
  "version": "0.1.0",
  "author": {{
    "name": "{}"
  }},
  "license": "MIT",
  "keywords": ["dstack-plugin"]
}}
"#,
        name, desc, author
    );
    fs::write(dir.join("plugin.json"), content)?;
    Ok(())
}

fn write_cursor_plugin(base: &Path, name: &str, desc: &str, author: &str) -> anyhow::Result<()> {
    let dir = base.join(".cursor-plugin");
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"{{
  "name": "{}",
  "displayName": "{}",
  "description": "{}",
  "version": "0.1.0",
  "author": {{
    "name": "{}"
  }},
  "license": "MIT",
  "skills": "./skills/",
  "commands": "./commands/",
  "hooks": "./hooks/hooks-cursor.json"
}}
"#,
        name, name, desc, author
    );
    fs::write(dir.join("plugin.json"), content)?;
    Ok(())
}

fn write_pawan_plugin(base: &Path, name: &str, desc: &str) -> anyhow::Result<()> {
    let dir = base.join(".pawan");
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"# {} plugin configuration for Pawan coding agent

[plugin]
name = "{}"
version = "0.1.0"
description = "{}"

[skills]
path = "../skills"

[commands]
path = "../commands"

[hooks]
session_start = "../hooks/session-start"
"#,
        name, name, desc
    );
    fs::write(dir.join("plugin.toml"), content)?;
    Ok(())
}

fn write_gemini_extension(base: &Path, name: &str, desc: &str) -> anyhow::Result<()> {
    let content = format!(
        r#"{{
  "name": "{}",
  "description": "{}",
  "version": "0.1.0",
  "contextFileName": "GEMINI.md"
}}
"#,
        name, desc
    );
    fs::write(base.join("gemini-extension.json"), content)?;
    Ok(())
}

fn write_gemini_md(base: &Path) -> anyhow::Result<()> {
    fs::write(
        base.join("GEMINI.md"),
        "@./skills/using-dstack/SKILL.md\n",
    )?;
    Ok(())
}

fn write_claude_md(base: &Path, name: &str) -> anyhow::Result<()> {
    let content = format!(
        "# {} Plugin\n\nThis plugin provides skills and hooks for AI-assisted development.\n\n## Available Skills\n\nRun `/skill list` to see available skills.\n",
        name
    );
    fs::write(base.join("CLAUDE.md"), content)?;
    Ok(())
}

fn write_agents_md(base: &Path, name: &str) -> anyhow::Result<()> {
    let content = format!(
        "# {} Plugin\n\nThis plugin provides skills and hooks for AI-assisted development.\n\n## Available Skills\n\nRun `/skill list` to see available skills.\n",
        name
    );
    fs::write(base.join("AGENTS.md"), content)?;
    Ok(())
}

fn write_changelog(base: &Path, name: &str) -> anyhow::Result<()> {
    let content = format!(
        "# Changelog\n\n## v0.1.0\n\n- Initial {} plugin scaffold\n- Generated by `dstack init`\n",
        name
    );
    fs::write(base.join("CHANGELOG.md"), content)?;
    Ok(())
}

fn write_codex_install(base: &Path, name: &str) -> anyhow::Result<()> {
    let dir = base.join(".codex");
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"# Installing {} for Codex

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/OWNER/{}.git ~/.codex/{}
   ```

2. **Create the skills symlink:**
   ```bash
   mkdir -p ~/.agents/skills
   ln -s ~/.codex/{}/skills ~/.agents/skills/{}
   ```

3. **Restart Codex** to discover the skills.

## Updating

```bash
cd ~/.codex/{} && git pull
```
"#,
        name, name, name, name, name, name
    );
    fs::write(dir.join("INSTALL.md"), content)?;
    Ok(())
}

fn write_opencode_install(base: &Path, name: &str) -> anyhow::Result<()> {
    let dir = base.join(".opencode");
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"# Installing {} for OpenCode

## Installation

Add to the `plugin` array in your `opencode.json`:

```json
{{
  "plugin": ["{}@git+https://github.com/OWNER/{}.git"]
}}
```

Restart OpenCode. The plugin auto-installs and registers all skills.
"#,
        name, name, name
    );
    fs::write(dir.join("INSTALL.md"), content)?;
    Ok(())
}

fn write_pawan_install(base: &Path, name: &str) -> anyhow::Result<()> {
    let dir = base.join(".pawan");
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"# Installing {} for Pawan

## Installation

```bash
mkdir -p ~/.config/pawan/plugins
git clone https://github.com/OWNER/{}.git ~/.config/pawan/plugins/{}
```

Or symlink:

```bash
ln -s /path/to/{}/plugin ~/.config/pawan/plugins/{}
```

Restart Pawan to discover skills.
"#,
        name, name, name, name, name
    );
    fs::write(dir.join("INSTALL.md"), content)?;
    Ok(())
}

fn write_opencode_shim(base: &Path, name: &str) -> anyhow::Result<()> {
    let dir = base.join(".opencode").join("plugins");
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"// {} plugin for OpenCode.ai
const path = require("path");
const fs = require("fs");

module.exports = {{
  name: "{}",
  version: "0.1.0",
  init(context) {{
    const skillsDir = path.join(path.dirname(__dirname), "..", "skills");
    if (context.registerSkillsPath) {{
      context.registerSkillsPath("{}", skillsDir);
    }}
  }},
  toolMapping: {{
    Bash: "bash", Read: "read", Edit: "edit",
    Write: "write", Glob: "glob", Grep: "grep",
  }},
}};
"#,
        name, name, name
    );
    fs::write(dir.join(format!("{}.js", name)), content)?;
    Ok(())
}

fn write_hooks(base: &Path) -> anyhow::Result<()> {
    let dir = base.join("hooks");
    fs::create_dir_all(&dir)?;

    // Claude Code hooks
    let hooks_json = r#"{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|clear|compact",
        "hooks": [
          {
            "type": "command",
            "command": "./hooks/session-start"
          }
        ]
      }
    ]
  }
}
"#;
    fs::write(dir.join("hooks.json"), hooks_json)?;

    // Cursor hooks
    let cursor_hooks = r#"{
  "version": 1,
  "hooks": {
    "sessionStart": [
      {
        "command": "./hooks/session-start"
      }
    ]
  }
}
"#;
    fs::write(dir.join("hooks-cursor.json"), cursor_hooks)?;

    // Session start script
    let session_start = r#"#!/bin/bash
# Session bootstrap — loaded at conversation start
echo '{"hookSpecificOutput": "dstack plugin loaded. Use /skill list to see available skills."}'
"#;
    fs::write(dir.join("session-start"), session_start)?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(dir.join("session-start"), fs::Permissions::from_mode(0o755))?;
    }

    Ok(())
}

fn write_skeleton_skill(base: &Path, name: &str) -> anyhow::Result<()> {
    let skill_name = format!("using-{}", name);
    let dir = base.join("skills").join(&skill_name);
    fs::create_dir_all(&dir)?;
    let content = format!(
        r#"---
name: using-{}
description: Session orientation — available skills and commands
---

# Using {}

This plugin provides skills for AI-assisted development.

## Available Skills

Use the Skill tool to invoke any skill by name.

## Getting Started

Start by describing what you want to accomplish. The plugin will suggest relevant skills.
"#,
        name, name
    );
    fs::write(dir.join("SKILL.md"), content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_platforms_count() {
        assert_eq!(PLATFORMS.len(), 6);
        assert!(PLATFORMS.contains(&"claude-code"));
        assert!(PLATFORMS.contains(&"cursor"));
        assert!(PLATFORMS.contains(&"pawan"));
        assert!(PLATFORMS.contains(&"codex"));
        assert!(PLATFORMS.contains(&"opencode"));
        assert!(PLATFORMS.contains(&"gemini"));
    }

    #[test]
    fn test_init_plugin_creates_all_files() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("my-plugin");
        init_plugin(dir.to_str().unwrap(), "test-plugin", "A test plugin", "testauthor").unwrap();

        // Core files
        assert!(dir.join("package.json").exists());
        assert!(dir.join("CLAUDE.md").exists());
        assert!(dir.join("AGENTS.md").exists());
        assert!(dir.join("CHANGELOG.md").exists());
        assert!(dir.join("GEMINI.md").exists());

        // Platform manifests
        assert!(dir.join(".claude-plugin/plugin.json").exists());
        assert!(dir.join(".cursor-plugin/plugin.json").exists());
        assert!(dir.join(".pawan/plugin.toml").exists());
        assert!(dir.join("gemini-extension.json").exists());

        // Install docs
        assert!(dir.join(".codex/INSTALL.md").exists());
        assert!(dir.join(".opencode/INSTALL.md").exists());
        assert!(dir.join(".pawan/INSTALL.md").exists());
        assert!(dir.join(".opencode/plugins/test-plugin.js").exists());

        // Hooks
        assert!(dir.join("hooks/hooks.json").exists());
        assert!(dir.join("hooks/hooks-cursor.json").exists());
        assert!(dir.join("hooks/session-start").exists());

        // Skeleton skill
        assert!(dir.join("skills/using-test-plugin/SKILL.md").exists());
    }

    #[test]
    fn test_package_json_content() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("pkg-test");
        init_plugin(dir.to_str().unwrap(), "my-tool", "Does things", "alice").unwrap();

        let content = fs::read_to_string(dir.join("package.json")).unwrap();
        assert!(content.contains(r#""name": "my-tool""#));
        assert!(content.contains(r#""description": "Does things""#));
        assert!(content.contains(r#""author": "alice""#));
        assert!(content.contains(r#""keywords": ["dstack-plugin"]"#));

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["name"], "my-tool");
        assert_eq!(parsed["version"], "0.1.0");
    }

    #[test]
    fn test_claude_plugin_json_valid() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("claude-test");
        init_plugin(dir.to_str().unwrap(), "cp-test", "Claude plugin", "bob").unwrap();

        let content = fs::read_to_string(dir.join(".claude-plugin/plugin.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["name"], "cp-test");
        assert_eq!(parsed["description"], "Claude plugin");
        assert_eq!(parsed["author"]["name"], "bob");
    }

    #[test]
    fn test_cursor_plugin_json_valid() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("cursor-test");
        init_plugin(dir.to_str().unwrap(), "cur-test", "Cursor plugin", "carol").unwrap();

        let content = fs::read_to_string(dir.join(".cursor-plugin/plugin.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["name"], "cur-test");
        assert_eq!(parsed["displayName"], "cur-test");
        assert_eq!(parsed["hooks"], "./hooks/hooks-cursor.json");
    }

    #[test]
    fn test_pawan_plugin_toml_valid() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("pawan-test");
        init_plugin(dir.to_str().unwrap(), "pw-test", "Pawan plugin", "dave").unwrap();

        let content = fs::read_to_string(dir.join(".pawan/plugin.toml")).unwrap();
        assert!(content.contains(r#"name = "pw-test""#));
        assert!(content.contains(r#"description = "Pawan plugin""#));
        assert!(content.contains(r#"path = "../skills""#));
    }

    #[test]
    fn test_gemini_extension_json_valid() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("gemini-test");
        init_plugin(dir.to_str().unwrap(), "gem-test", "Gemini ext", "eve").unwrap();

        let content = fs::read_to_string(dir.join("gemini-extension.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["name"], "gem-test");
        assert_eq!(parsed["contextFileName"], "GEMINI.md");
    }

    #[test]
    fn test_hooks_session_start_executable() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("hooks-test");
        init_plugin(dir.to_str().unwrap(), "h-test", "Hooks test", "frank").unwrap();

        let script = dir.join("hooks/session-start");
        assert!(script.exists());
        let content = fs::read_to_string(&script).unwrap();
        assert!(content.starts_with("#!/bin/bash"));
        assert!(content.contains("hookSpecificOutput"));

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::metadata(&script).unwrap().permissions();
            assert!(perms.mode() & 0o111 != 0, "session-start should be executable");
        }
    }

    #[test]
    fn test_skeleton_skill_has_frontmatter() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("skill-test");
        init_plugin(dir.to_str().unwrap(), "sk-test", "Skill test", "grace").unwrap();

        let content = fs::read_to_string(dir.join("skills/using-sk-test/SKILL.md")).unwrap();
        assert!(content.starts_with("---\n"));
        assert!(content.contains("name: using-sk-test"));
        assert!(content.contains("description: Session orientation"));
        assert!(content.contains("# Using sk-test"));
    }

    #[test]
    fn test_init_dry_run_creates_nothing() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("dry-run-test");
        fs::create_dir_all(&dir).unwrap();

        init_dry_run(dir.to_str().unwrap(), "dry-test");

        // Directory should still be empty — dry run writes nothing
        let entries: Vec<_> = fs::read_dir(&dir).unwrap().collect();
        assert!(entries.is_empty(), "dry run should not create any files");
    }

    #[test]
    fn test_opencode_shim_js_content() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("shim-test");
        init_plugin(dir.to_str().unwrap(), "oc-test", "OC test", "hank").unwrap();

        let content = fs::read_to_string(dir.join(".opencode/plugins/oc-test.js")).unwrap();
        assert!(content.contains(r#"name: "oc-test""#));
        assert!(content.contains("module.exports"));
        assert!(content.contains("registerSkillsPath"));
        assert!(content.contains("toolMapping"));
    }

    #[test]
    fn test_name_with_special_chars() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("special-test");
        // Names with hyphens and numbers should work fine
        init_plugin(dir.to_str().unwrap(), "my-plugin-2", "Test v2", "user-1").unwrap();
        assert!(dir.join("package.json").exists());
        assert!(dir.join("skills/using-my-plugin-2/SKILL.md").exists());
    }

    #[test]
    fn test_changelog_mentions_plugin_name() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("cl-test");
        init_plugin(dir.to_str().unwrap(), "changelog-test", "CL", "author").unwrap();

        let content = fs::read_to_string(dir.join("CHANGELOG.md")).unwrap();
        assert!(content.contains("# Changelog"));
        assert!(content.contains("changelog-test"));
        assert!(content.contains("dstack init"));
    }
}
