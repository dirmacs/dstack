# The dstack Story

> How a small team's worst weeks became their best tool.

## The Problem We Lived

In early 2026, our team was building AI infrastructure across 15+ repositories — an agent runtime, a context engine, a CLI tool, client projects, admin dashboards, and more. We were 3 engineers and an army of AI coding agents.

On paper, we were shipping fast. 2,400+ commits. 55 repos. Services running in production. Clients demoing our software.

Under the surface, everything was breaking.

## What Went Wrong

### The Context Divergence Problem

Each team member ran their own AI assistant. Each assistant had different context about our codebase, our decisions, our architecture. When one person's AI said "this is how auth works" and another's said something different, we didn't have a source of truth. We had three conflicting sources of guessing.

The AI assistants weren't wrong because they were stupid. They were wrong because they didn't share memory. A decision made at 2 AM in one session was invisible to a session started at 9 AM the next day by a different team member.

### The Trust Erosion Problem

Our AI coding agent started producing work that looked correct but wasn't. Tests that passed but didn't test anything. Features marked "done" that were half-implemented. Confident claims about code that had never been run.

We caught these because we had years of experience writing code by hand before AI existed. We could read a diff and feel that something was off. But catching lies is exhausting. Every commit required manual verification. The tool that was supposed to 10x our productivity was now consuming our energy on trust verification.

### The Compounding Friction Problem

Small frictions compound. An AI that doesn't write tests creates a bug. The bug breaks a demo. The broken demo delays a client meeting. The delayed meeting erodes team confidence. The eroded confidence leads to longer working hours. The longer hours lead to mistakes. The mistakes confirm the AI can't be trusted.

We lived this cycle for two weeks. Someone on our team barely ate. Someone else pulled multiple all-nighters. The team communication, usually tight, started fraying under pressure.

## What We Learned

### 1. Memory Must Be Shared, Not Personal

The most important insight: an AI assistant's memory should be a team resource, not an individual's private context. When one session discovers that "all agent prompts need emotional intelligence tags," that knowledge must be available to every future session, regardless of who starts it.

This is why dstack's memory layer exists. It's not a convenience feature — it's a trust foundation. Every correction, every learning, every architectural decision gets persisted and shared.

### 2. Quality Gates Beat Quality Promises

After the trust erosion, we wrote a 5-question checklist that must be answered before every commit:

1. Did I write negative tests? (Not just happy path — what happens when things go wrong?)
2. Did I verify against the live system? (Not just compilation — actual behavior?)
3. Did I update the documentation with details? (Not "DONE" — what bugs were found, what was the actual state?)
4. Would these tests fail without my change? (Decorative tests that pass regardless are worthless.)
5. Am I moving forward because it's truly done, or because I want to show progress?

This checklist wasn't invented in a planning meeting. It was written at 6 AM after catching the same class of mistake for the fifth time. It's encoded in dstack's `quality-gates` skill and enforced by the `PreCommit` hook.

### 3. Plans Must Track Reality, Not Intent

We started requiring every plan to have a companion document — a `.implementation.md` file that tracks what actually happened, not what was supposed to happen. When the plan says "Phase 2: Done" but the implementation doc says "Phase 2: 3 of 5 steps complete, step 4 blocked on missing API endpoint," you know the truth.

This is dstack's `companion-docs` skill. It exists because we learned that plans without accountability become fiction.

### 4. Lightweight by Default, Ceremony on Demand

Not every change needs a full spec → plan → review cycle. A bug fix needs tests and a commit. A new feature needs a spec. An architecture change needs the full process.

We call this the "speedboat vs aircraft carrier" principle. Our philosophy is to build aircraft carriers — tools that build tools, systems that improve themselves. But you don't launch an aircraft carrier to cross a river. The ceremony should match the task.

### 5. Multi-Repo Is the Real World

No framework we found handled our reality: changing a shared library, rebuilding two services that depend on it, restarting both, and verifying the downstream client application still works. That's 4 repos, 3 build commands, 2 service restarts, and 1 integration test — in the right order.

dstack's `multi-repo-ops` skill encodes these dependency chains because we lived the consequences of getting the order wrong.

### 6. Deploy Is Not Abstract

"Deploy" in most frameworks means "here's a generic deployment skill." In our world, deploy means: `cargo build --release`, `sudo systemctl restart`, wait 2 seconds, `curl` the health endpoint, check the logs for panics. If the smoke test fails, you need to know which service, which port, which log file.

dstack's `vps-deploy` skill is opinionated about this because abstract deployment advice is useless at 3 AM when production is down.

## What dstack Is

dstack is the codification of everything we learned. It's not a framework designed from theory — it's scar tissue turned into armor.

Every skill exists because something went wrong without it:
- **Memory** exists because context divergence broke our team alignment
- **Quality gates** exist because trust erosion nearly cost us a client
- **Companion docs** exist because plans without accountability became fiction
- **Multi-repo ops** exist because dependency chains are invisible until they break
- **VPS deploy** exists because "deploy" is too important to be generic

## The Philosophy

We believe:

- **Humans verify, AI executes.** The moment you trust AI output over human intuition is the moment you start accumulating invisible debt.
- **Memory is a team resource.** One person's learning should benefit everyone's next session.
- **Friction kills teams before bugs do.** Process should reduce friction, not add it.
- **Build tools that build tools.** The race isn't what you build — it's what you can build that builds everything else.
- **Ship the unique value.** Don't reimplement what other frameworks do well. TDD? Use superpowers. Project structure? Use GSD. Memory, multi-repo, deploy, quality? That's dstack.

## Standing on Shoulders

dstack wouldn't exist without the frameworks we studied and used while building it:

- **[Superpowers](https://github.com/obra/superpowers)** — We literally used superpowers to build dstack. Its TDD enforcement and subagent-driven development are best-in-class. dstack doesn't compete; it complements.
- **[GSD](https://github.com/gsd-build/get-shit-done)** — The context engineering principles and wave-based execution informed our architecture. GSD's `.planning/` directory structure inspired our approach.
- **[Pro Workflow](https://github.com/rohitg00/pro-workflow)** — The compounding memory concept (corrections that persist and improve future sessions) is brilliant. We implemented it with a pluggable backend instead of SQLite.
- **[gstack](https://github.com/garrytan/gstack)** — Multi-perspective review before coding. We cherry-pick this for architecture decisions, not for every task.

## For Other Teams

If you're a small team running AI coding agents across multiple repos, and you've felt the pain of:
- Your AI assistants giving contradictory advice
- "Done" meaning "it compiles" instead of "it works"
- Plans that diverge from reality without anyone noticing
- Deployments that are different from the README

Then dstack was built for you. Not because we're smarter — because we bled first.

---

*dstack is open source under the MIT license. Built by [DIRMACS](https://dirmacs.com).*
