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
