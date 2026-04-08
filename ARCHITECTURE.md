# dstack — DIRMACS Development Stack

> An opinionated Claude Code skills stack for multi-repo Rust-first AI infrastructure development. Born from 2 weeks of real production work across 15+ repos.

## What dstack IS

dstack is DIRMACS's answer to superpowers/GSD/gstack/BMAD — but built from lived experience, not theory. It's a Claude Code plugin (skills + hooks + commands) that encodes the workflows, quality gates, and delegation patterns we've battle-tested across Eruka, ARES, eHB, pawan, and doltares.

## Landscape Analysis

| Framework | Strength | Weakness | What dstack takes from it |
|-----------|----------|----------|---------------------------|
| **Superpowers** | TDD enforcement, subagent-driven dev, auto-skill activation | No persistent memory, no multi-repo awareness | Skill activation patterns, TDD gating, verification-before-completion |
| **GSD** | Context engineering (anti-rot), wave-based parallel execution, fresh context per task | Heavyweight ceremony for small tasks, no memory | `.planning/` structure, wave execution, STATE.md concept |
| **gstack** | Multi-perspective review (CEO/eng/design), decision quality | Token-expensive, overkill for implementation | Selective review gates for architecture decisions only |
| **Pro Workflow** | SQLite-backed self-correcting memory, compounding over 50+ sessions | No spec-driven dev, no multi-repo | Memory compounding concept (but we use Eruka instead of SQLite) |
| **Compound Engineering** | 80/20 plan/execute ratio, learning capture | Light on implementation | `/compound` learning capture pattern |
| **Night Market** | 23 granular plugins, multi-LLM delegation (conjure) | Too many moving parts, plugin sprawl | Cherry-pick: conjure (NIM delegation), conserve (token mgmt), egregore (parallel agents) |
| **Legion** | 48 specialist agents, wave execution, plan contracts | Overkill personality system | Wave execution + file-overlap detection |
| **Spec-kit/Kiro** | Spec-as-source aspiration | Specs become review burden, agents ignore them | Light spec-first for new features only |

## dstack's Unique Advantages

Things none of the above have:

1. **Eruka-backed persistent memory** — Not SQLite, not flat files. Real context engine with field-level confidence, knowledge states, workspace scoping. Memory that's queryable, mergeable, and shared across sessions AND agents.

2. **Multi-repo orchestration native** — 15 repos at `/opt/`, cross-repo dependency awareness (ares↔dirmacs-core↔eruka↔ehb). No other framework handles "change dirmacs-core, rebuild ares-dirmacs, restart service, verify ehb still works."

3. **pawan delegation** — Native integration with pawan CLI agent for NIM-model background tasks. Claude reasons, pawan executes. Two-tier: Claude for architecture, pawan for mechanical implementation.

4. **Production VPS awareness** — systemctl, Caddy, PostgreSQL, journalctl. Skills that know how to deploy, not just how to code.

5. **Companion doc pattern** — Every plan gets a `.implementation.md` that tracks reality vs intent. No other framework does this.

## Architecture

```
dstack/
├── CLAUDE.md                    # Plugin bootstrap (loaded on session start)
├── package.json                 # Plugin metadata
├── hooks/
│   ├── hooks.json               # Claude Code hook definitions
│   └── session-start            # Load Eruka context + active skills
├── skills/
│   ├── 00-using-dstack/         # Meta: how to use dstack
│   │   └── SKILL.md
│   ├── 01-spec-first/           # Lightweight spec before code
│   │   └── SKILL.md
│   ├── 02-impl-test-pairs/      # TDD: impl + tests always together
│   │   └── SKILL.md
│   ├── 03-multi-repo-ops/       # Cross-repo build/test/deploy
│   │   └── SKILL.md
│   ├── 04-eruka-memory/         # Read/write Eruka context in workflow
│   │   └── SKILL.md
│   ├── 05-pawan-delegation/     # Delegate to pawan for background tasks
│   │   └── SKILL.md
│   ├── 06-companion-docs/       # Plan ↔ implementation doc pairing
│   │   └── SKILL.md
│   ├── 07-wave-execution/       # Parallel task waves with deps
│   │   └── SKILL.md
│   ├── 08-vps-deploy/           # systemctl, Caddy, PostgreSQL ops
│   │   └── SKILL.md
│   ├── 09-quality-gates/        # 5-question pre-commit checklist
│   │   └── SKILL.md
│   ├── 10-git-discipline/       # Authorship, branch sync, atomic commits
│   │   └── SKILL.md
│   ├── 11-debug-systematic/     # 4-phase root cause analysis
│   │   └── SKILL.md
│   ├── 12-nim-model-triage/     # Three-phase NIM model evaluation
│   │   └── SKILL.md
│   └── 13-context-engineering/  # Token management, fresh context per task
│       └── SKILL.md
├── commands/
│   ├── plan.md                  # /plan — create implementation plan
│   ├── execute.md               # /execute — wave-based task execution
│   ├── verify.md                # /verify — E2E verification against live
│   ├── deploy.md                # /deploy — build + restart + smoke test
│   ├── sync.md                  # /sync — vps-git-sync all repos
│   ├── delegate.md              # /delegate — dispatch to pawan
│   └── audit.md                 # /audit — scan for stale/missing work
└── templates/
    ├── PLAN.md                  # Plan template
    ├── IMPLEMENTATION.md        # Companion doc template
    └── TASK.xml                 # GSD-style atomic task template
```

## Design Philosophy

Born from two weeks of real pain. Every principle exists because ignoring it cost us something.

### Core Beliefs

1. **Humans verify, AI executes.** The moment you trust AI output over human intuition is the moment you accumulate invisible debt. dstack enforces verification, not just generation.

2. **Memory is a team resource, not a personal diary.** Context divergence between team members' AI instances caused more damage than any bug. One person's learning must benefit everyone's next session. This is why the memory layer exists and why it's shareable.

3. **Friction kills teams before bugs do.** Process friction + unreliable AI + communication gaps compound exponentially. Every feature in dstack is evaluated against: does this reduce friction or add it?

4. **Build tools that build tools.** The race isn't what you build — it's what you can build that builds everything else. dstack is an aircraft carrier, not a speedboat. But you don't launch an aircraft carrier to cross a river.

5. **Ship the unique value.** Don't reimplement what other frameworks do well. TDD? Superpowers. Project structure? GSD. Memory, multi-repo, deploy, quality gates? That's dstack.

### Skill Design Principles

1. **Auto-activate, don't require invocation** — Skills trigger based on context, like superpowers. When touching Rust code → `impl-test-pairs` activates. When touching multiple repos → `multi-repo-ops` activates.

2. **Pluggable memory, not vendor lock-in** — MemoryProvider trait with FileProvider (default) and ErukaProvider (power-up). Others can add Redis, SQLite, whatever. Memory works everywhere; Eruka makes it exceptional.

3. **Lightweight by default, ceremony on demand** — Bug fix? Tests + commit. New feature? Spec + plan + execution. Architecture change? Full review cycle. The ceremony matches the task.

4. **Production-aware, not production-abstract** — Every skill knows about real infrastructure. `deploy` isn't generic advice — it's `cargo build --release && sudo systemctl restart && curl health endpoint`. Because abstract deployment advice is useless at 3 AM.

5. **Compound, don't accumulate** — Learnings from each session feed back into memory. Next session starts with accumulated context. Corrections compound; mistakes don't repeat.

## Hook System

```json
{
  "hooks": {
    "SessionStart": [{
      "matcher": "startup|clear|compact",
      "hooks": [{
        "type": "command",
        "command": "dstack session-start",
        "async": false
      }]
    }],
    "PreCommit": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "dstack quality-gate",
        "async": false
      }]
    }]
  }
}
```

**SessionStart**: Load Eruka context for current project, activate relevant skills based on directory.
**PreCommit**: Run 5-question quality checklist, block commit if answers are "no".

## Migration Path

### Phase 0: Extract from existing (THIS WEEK)
- Extract 15 existing `~/.claude/skills/` into dstack skill format
- Extract patterns from CLAUDE.md into skill files
- Extract memory patterns from Eruka integration

### Phase 1: Core skills (NEXT)
- Implement skills 00-06 (the essentials)
- Hook system (session start, pre-commit)
- Commands: /plan, /execute, /verify

### Phase 2: Advanced (LATER)
- Wave execution engine
- pawan delegation protocol
- NIM model triage
- Multi-LLM delegation (Night Market's conjure concept)

### Phase 3: Polish + Open Source
- Documentation
- Installation script
- Plugin marketplace submission
- Blog post: "Why we built dstack"

## What dstack is NOT

- Not a project management tool (no Jira, no sprints)
- Not a framework for beginners (assumes Rust, PostgreSQL, VPS)
- Not a replacement for thinking (it enforces quality, not generates ideas)
- Not vendor-locked to Claude (skills are portable to pawan, opencode)
