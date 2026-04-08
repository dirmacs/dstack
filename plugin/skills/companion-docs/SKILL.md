---
name: companion-docs
description: Track implementation reality vs plan intent with companion documents
---

# Companion Docs

Every plan gets a `.implementation.md` companion that tracks what actually happened.

## When to Create

Create a companion doc when:
- Starting work from a plan or spec
- Beginning a multi-step feature implementation
- Running a sprint or audit

## Naming Convention

```
<plan-name>.implementation.md
```

Lives in the same directory as the plan it tracks.

## Template

```markdown
# [Plan Name] — Implementation Log

**Plan:** `<link to plan file>`
**Started:** YYYY-MM-DD
**Status:** IN PROGRESS | DONE | BLOCKED

## Progress

### Phase/Step N: [Name]
- **Status:** DONE | IN PROGRESS | BLOCKED | SKIPPED
- **Commit:** `<hash>` — <message>
- **Details:** What actually happened, bugs found, deviations from plan
- **Tests:** N passing, N failing

## Deviations from Plan

- [What changed and WHY]

## Bugs Found

- [Bug description, root cause, fix]

## Open Questions

- [Anything unresolved]
```

## Rules

1. **Update after every commit** — not at the end of the session
2. **Include DETAILS** — commit hashes, test counts, bugs found, DB state
3. **Note deviations** — plans never survive contact with reality; document what changed
4. **Don't let docs rot** — if a companion doc is >7 days old without update, it's stale

## Stale Detection

```bash
dstack audit --stale
```

Scans tracked repos for `.implementation.md` files older than 7 days. Stale docs indicate abandoned work or missing updates.
