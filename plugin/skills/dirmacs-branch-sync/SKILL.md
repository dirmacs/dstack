---
name: dirmacs-branch-sync
description: Sync holy ↔ main/master across dirmacs repos, delete stray branches with zero unique commits, and enforce the dual-branch convention selectively (opt-in list, not blanket).
allowed-tools: Bash, Read, Write, Edit, Glob, Grep
argument-hint: "[--dry-run] [--repo <name>] [--skip-enforce]"
---

# Skill: dirmacs-branch-sync

Keeps dirmacs repos healthy across the `holy` (WIP) and `main`/`master` (stable) branch pair: fast-forwards either direction, deletes strays with zero unique work, and — **selectively** — ensures both branches exist in repos that follow the convention.

## When to Use

- End-of-session housekeeping after a multi-repo push
- Before a release cut to make sure `main`/`master` carries every commit from `holy`
- When `git branch -r` shows unfamiliar branches and you want to know which are safe to delete
- After onboarding a new dirmacs repo, to make sure it has the expected branch layout

## The Dirmacs Convention

Most dirmacs **product** repos use a **dual-branch** layout:

| Branch | Role |
|--------|------|
| `holy` | WIP / daily driver. All session work lands here first. |
| `main` | Stable. Fast-forwarded from `holy` when `holy` is ready. |

Published OSS crates and forks follow upstream convention instead (usually `master`). The skill respects this — it does **not** force `main` everywhere.

## Selectivity Rule (CRITICAL)

**Never apply the enforcement to every repo blindly.** The user explicitly corrected this: "for the repos to which this applies / not all repos."

### Decision tree for each repo

```
Is the repo in the DIRMACS_PRODUCT_ALLOWLIST?
├── Yes → canonical = "main", apply full enforcement (ensure holy + main both exist)
└── No → Does the repo have an existing `holy` branch?
         ├── Yes → it opted in at some point
         │        Does it also have `main`? → leave alone, sync holy↔main
         │        Does it also have `master`? → leave alone, sync holy↔master (OSS crate pattern)
         │        Neither? → DO NOT auto-create main. Flag for user review.
         └── No → this repo is not on the convention. Skip entirely.
```

### Allowlist (canonical name = "main")

Dirmacs product repos that get the full holy+main enforcement:

```
ares            # ARES runtime
eruka           # context engine
ehb             # eHealthBuddy
doltares        # orchestration daemon
doltclaw        # minimal Rust agent runtime
pawan           # CLI coding agent
pawan-ui        # pawan web UI (private)
deagle          # code intelligence
dstack          # dev stack tooling (product repo, not the OSS plugin)
dirmacs-site    # marketing site
dirmacs-admin   # admin dashboard
dirmacs-web     # public web components
holy-doc        # HLD documentation
things-to-do    # LLD execution plans
da-execution-plans  # LLD templates
ehb-admin       # eHB admin UI
ehb-buddy       # eHB companion UI
ehb-listener    # eHB listener agent
dirmacs-skills  # reusable skill library
enterprise-portal   # Leptos portal
dotdot-v2       # DOT DOT marketplace
kasino          # product repo
nimakai         # NIM benchmarker
channel-gateway # WhatsApp bridge (private)
doltdot         # automation scripts
ares-config     # ARES config (private, symlinked)
aegis           # config manager
lumivid         # video inference project
```

### Explicit opt-out (canonical name = "master", leave alone)

These are published OSS crates or upstream-tracking forks. `holy` may exist as a side branch but `master` is canonical — do not create `main`:

```
thulp           # published crate, master canonical
thulpoff        # published crate, master canonical
lancor          # published crate, master canonical
dwasm           # published tool, master canonical
daedra          # published crate, master canonical
```

### Per-repo override

A repo can declare its own canonical branch by dropping a `.dstack/branches.toml` file at its root:

```toml
# .dstack/branches.toml
canonical = "main"        # or "master" or "none"
enforce_dual = true       # if true, ensure `holy` + `canonical` both exist
stray_allowlist = ["release-*", "hotfix-*"]   # never delete these even if they look stray
```

The skill reads this file first; allowlists above are fallbacks.

## Workflow

### 1. Audit phase (read-only)

For each repo in `/opt/*/.git`:

```bash
cd /opt/<repo>
git fetch --all --prune --quiet
git branch -a --format='%(refname:short)' | grep -v '^origin$'
```

Compute for each repo:
- **Canonical name**: `main` if in allowlist; `master` if in opt-out list; read `.dstack/branches.toml` if present; otherwise infer from `origin/HEAD`.
- **Has holy?** — local and remote.
- **Has canonical?** — local and remote.
- **Divergence**: `git rev-list --count <canonical>..holy` and `git rev-list --count holy..<canonical>`.
- **Strays**: any `origin/*` branch that is not `holy`, `main`, `master`, or `HEAD`. Exclude names matching `stray_allowlist`.

Print a per-repo table. Nothing is mutated yet.

### 2. Sync phase

For each repo where both `holy` and `canonical` exist:

```
holy is ahead of canonical by N, canonical is 0 ahead of holy:
    → fast-forward canonical to holy
    → push canonical

canonical is ahead of holy by M, holy is 0 ahead of canonical:
    → fast-forward holy to canonical (rare — usually happens when a parallel session pushed to main directly)
    → push holy

both diverged (holy ahead by N, canonical ahead by M):
    → STOP. Do not auto-merge. Print a warning.
    → User must rebase/merge manually.

both identical:
    → no-op
```

**Never use `--force`.** Never rebase. Never auto-resolve conflicts.

### 3. Stray cleanup phase

For each `origin/<stray>` branch flagged in the audit:

```bash
# Count unique commits vs both main and holy
unique_vs_main=$(git rev-list --count <canonical>..origin/<stray>)
unique_vs_holy=$(git rev-list --count holy..origin/<stray>)

if unique_vs_main == 0 AND unique_vs_holy == 0:
    → safe to delete. Delete local + remote.

else:
    → PRINT the commits. Ask user before deleting.
    → NEVER delete branches with unique work without explicit user approval.
```

### 4. Enforcement phase (SELECTIVE)

For each repo **in the allowlist** (or with `enforce_dual = true` in its `.dstack/branches.toml`):

```
If canonical branch is missing locally:
    → create it from holy: `git branch <canonical> holy`
    → push it: `git push -u origin <canonical>`

If holy is missing and canonical exists:
    → create holy from canonical: `git branch holy <canonical>`
    → push it
```

**Do NOT run this phase on repos outside the allowlist.** That was the correction — published crates that use `master` must not have `main` created against their will.

### 5. Report

Print a summary:

```
Repos audited:   N
  - in sync:     X
  - synced:      Y  (fast-forwarded)
  - diverged:    Z  (manual intervention needed)
  - enforced:    W  (missing branch created)
Strays deleted:  S
Strays kept:     K  (had unique work)
```

## Invariants

- Never force-push. Never rewrite history.
- Never create `main` on a repo with `canonical = "master"`.
- Never delete a branch with unique commits without user approval.
- Always fetch before computing divergence — stale local refs lie.
- Always verify the commit author after any auto-commit: `git log -1 --format="%an <%ae>"` should be `bkataru <baalateja.k@gmail.com>`.
- `origin/HEAD` shows up as literal `origin` in `git branch -r --format='%(refname:short)'` output — filter it out before stray detection.

## Dry-run output

`--dry-run` prints the full plan (sync operations + stray deletions + enforcement actions) without executing any mutating command. Always run with `--dry-run` first on unfamiliar repo sets.

## Example output

```
$ dirmacs-branch-sync --dry-run

/opt/eruka       holy +3 / main 0    → would fast-forward main to holy (push)
/opt/ares        holy 0 / main +3    → would fast-forward holy to main (push)
/opt/ehb         holy +0 / main +0    in sync
/opt/thulpoff    master canonical, holy absent    skip (OSS crate, not in allowlist)
/opt/dirmacs-web holy exists, main MISSING   → would create main from holy (in allowlist)
/opt/things-to-do   holy +1 / main +2 DIVERGED    manual merge needed (WARNING)

Strays:
/opt/pawan       origin/sparrow-4    0 unique vs main, 0 unique vs holy   → would delete
/opt/eruka       origin/feature-x    5 unique vs main, 2 unique vs holy   → KEEP (user review)
```

## Integration

- Runs standalone via `Bash`
- Complements `vps-git-sync` (that one commits dirty files; this one handles branches)
- Can be called from a `dstack audit` hook before release cuts
- Installed to `/root/.claude/skills/dirmacs-branch-sync/` on the VPS for self-use
- Also ships in:
  - `github.com/dirmacs/skills` → `dirmacs-branch-sync/SKILL.md`
  - `github.com/dirmacs/dstack` → `plugin/skills/dirmacs-branch-sync/SKILL.md`

## Why the selectivity rule matters

Earlier in this session, Claude ran a blanket "create main in every repo without one" script across the VPS. bkataru interrupted immediately:

> "for the repos to which this applies / add this to workflow/skill that you'll make out of it / not all repos"

The correction is load-bearing: **dirmacs has two branch conventions, not one**. Product repos use `holy` + `main`. Published OSS crates use `master` because that's what crates.io, docs.rs, and downstream users expect. Forcing `main` onto `thulpoff` would have:
- Created a confusing divergence between VCS state and published crate
- Broken upstream tracking for any fork-aware tooling
- Required another round trip to undo

The allowlist + opt-out list above encodes this directly. The skill refuses to touch repos outside it, even if running with `--force` flags.
