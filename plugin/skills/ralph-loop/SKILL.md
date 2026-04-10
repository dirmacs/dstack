---
name: ralph-loop
description: Use when you want an agent to work autonomously and indefinitely on a long-running project without stalling out. Sets up multiple parallel cron loops on co-prime cadences (3m/7m/11m) that handle execution, quality, and intel work, with a hard anti-idle rule and fallback hierarchy that regenerates tasks from context when the queue empties.
---

# Ralph Loop — Autonomous Iteration Pattern

Named after the "Ralph Wiggum loop" — simple, persistent, never gives up.
Turns a passive agent into a perpetual-motion work machine that keeps
iterating on a project until the session expires.

## When to use

- **Long-running improvement projects** that don't fit in one session —
  multi-day integrations, test coverage expansion, library migrations,
  documentation overhauls
- **Dogfooding sessions** where you want the agent to live inside a
  codebase and improve it continuously
- **Walkaway work** — meaningful progress happens without you sitting
  there driving every decision
- **Multi-phase projects** with a known list of sub-goals

**Do NOT use** for small tasks (≤1 hour), one-shot questions, or any
task where step-by-step user feedback matters more than autonomy.

## Core rule: anti-idle

> **Every loop tick MUST produce something concrete.** Never just report
> status. If you have no assigned task, CREATE one from the environment
> (commits, memory, TaskList, project docs, recent file changes).

Traditional agents stop when their task is done. Ralph loops keep going
until they exhaust the universe of things to improve — which, for any
real project, is effectively never.

## The three loops (co-prime cadences)

| Loop | Cadence | Concern | Fallback if idle |
|---|---|---|---|
| **Execution** | 3m | Main work — pick next task, do it, commit | Pick biggest pending TODO |
| **Quality** | 7m | Tests — add 3-5 meaningful tests per tick | Rotate through modules |
| **Intel** | 11m | Docs, memory, architecture audit | Rotate through 7 options |

Why co-prime? 3, 7, 11 have no common factors → they rarely fire
simultaneously, distributing work across time. 3×7×11 = 231 minutes
between full syncs, so each loop gets independent progress.

## Fallback hierarchy

The magic of ralph loops is that they never stall. When a loop has
nothing to do, it walks this hierarchy until it finds work:

1. **TaskList pending items** — pick highest priority not-blocked
2. **Test coverage gaps** — always valid, always improves quality
3. **Documentation drift** — check docs vs git log, update stale sections
4. **Memory refresh** — journal new patterns to memory files
5. **Intel analysis** — run code intelligence, find dead code, audit
6. **Brainstorm** — read recent commits, project context, user messages,
   extract latent work, create new TaskList items
7. **Last resort** — spawn a subagent with a vague goal like "audit
   this module for improvements"

**Never** just report "no work to do." That's a loop failure.

## Rotation — prevent tunnel vision

Each loop tick picks a DIFFERENT target from the previous tick:
- Quality loop rotates through modules
- Intel loop rotates through the 7 options
- Execution loop picks different phase numbers each tick

Rotation keeps the agent from drilling into one file to perfection
while the rest of the codebase rots.

## Setup

### Step 1: Seed the TaskList

Before starting the loops, populate the TaskList with 15-30 concrete
tasks. Ralph loops are efficient task consumers but bad cold-start
generators. Seed them with real phases, concrete tests to add, docs
to update.

### Step 2: Create the three loops

Each loop gets a prompt that:
- States the concern clearly (main work / tests / intel)
- Lists the fallback hierarchy specific to that loop
- Ends with "NEVER just report status — always DO work"
- References the project-specific context (repo path, key modules)

Set cadences at `*/3 * * * *`, `*/7 * * * *`, `*/11 * * * *`.

### Step 3: Walk away

The loops fire when the REPL is idle. Your conversation turns pause
the loops briefly but the loops resume between turns.

## Loop prompt templates

### Execution loop (3m)

```
Continue [PROJECT] work. Check TaskList for pending tasks. Pick the
highest-priority task that is not blocked. Work on it using:
1. Code intelligence tools when exploring unfamiliar code
2. Subagents for heavy parallel work
3. Commit each meaningful quantum with proper author

Guidelines:
- Every tick MUST produce a commit, doc update, test, or subagent spawn
- If all tasks complete, expand test coverage
- If tests exhausted, brainstorm new tasks from commits and docs
- NEVER just report status — always DO work

Process: check TaskList, pick work, execute, commit.
```

### Quality loop (7m)

```
Test expansion loop for [PROJECT]. Every tick:

1. Get baseline: run the test suite, note the count
2. Pick a module with low coverage (rotate — don't repeat last tick's pick)
3. Read the module, identify untested paths
4. Add 3-5 focused tests: happy path, error cases, edge cases, boundaries
5. Run the tests, verify they pass AND would fail without the code under test
6. Commit with `test: +N tests for <module>`

Use git log to see recent test commits and don't duplicate. Rotate
through modules — pick different ones each tick.
```

### Intel loop (11m)

```
Intel + docs loop. Every tick, do one of (rotate):

1. Dead code hunt: find public functions with no references
2. Architecture audit: check biggest types for SRP violations
3. Coupling analysis: find most-imported modules, identify hotspots
4. Doc update: verify docs/ against actual code, fix drift
5. Memory update: journal new patterns, gotchas, decisions
6. Intel refresh: check git log for unintegrated work
7. Cross-repo check: scan related repos for changes affecting this one

Do ONE option per tick. Commit any changes. Focus on actionable intel.
```

## Integration with dstack

dstack's thread memory layer makes ralph loops survive across session
boundaries:

```bash
# Capture persistent context before starting loops
dstack thread create "long-running-project"

# ... agent sets up 3 loops, walks away for 4h ...

# Loops continue across session restarts
dstack thread resume "long-running-project"
```

The dstack quality-gate hook runs automatically on each loop commit,
catching quality regressions before they accumulate.

## Anti-patterns

| ❌ Don't | ✅ Do |
|---|---|
| Let a tick pass without work | Always produce at least one commit or subagent spawn |
| Pick the same module every quality tick | Rotate — track the last picks |
| Report "nothing to do" | Walk the fallback hierarchy to find work |
| Run loops on 1m cadence | 3m minimum — 1m creates context churn |
| Use same minute for all loops (e.g. all *:00) | Co-prime intervals distribute better |
| Block on subagents | Run heavy work in background, continue other loops |
| Force-push main without permission | Never — even in loops |

## Minimal setup

For small projects, the minimal ralph loop is ONE loop on 5m cadence:

```
Continue work on [PROJECT]. Check git log for recent work, identify
the most obvious next step, do it, commit. If stuck, write tests.
Every tick MUST produce a commit. Never report status only.
```

Less structured, but the anti-idle rule alone delivers real value.

## Success metrics

A healthy ralph loop produces:
- **At least 1 commit per 3-minute tick** on the execution loop
- **3-5 tests per quality tick** on the 7m loop
- **1 doc/memory update per intel tick** on the 11m loop
- **Zero "no work to do" messages** — if you see one, the prompt
  needs a better fallback hierarchy
