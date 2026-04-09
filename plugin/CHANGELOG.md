# Changelog

## v0.2.0 (2026-04-09)

- Multi-platform plugin support (Claude Code, Cursor, Codex, OpenCode, Gemini CLI)
- 6 skills: using-dstack, eruka-memory, multi-repo-ops, vps-deploy, companion-docs, quality-gates
- 4 hooks: session-start, quality-gate, proprietary-guard, context-monitor
- 3 commands: /deploy, /sync, /audit
- Cursor-specific hooks format
- OpenCode.ai JavaScript plugin shim
- Codex symlink-based skill discovery
- Gemini CLI extension metadata

## v0.1.0 (2026-04-09)

- Initial release
- Claude Code plugin with hooks and skills
- Persistent memory via File + Eruka backends
- CLI: `dstack memory`, `dstack sync`, `dstack audit`, `dstack deploy`, `dstack skills`
- Quality gate pre-commit checklist
- Proprietary content detection hook
