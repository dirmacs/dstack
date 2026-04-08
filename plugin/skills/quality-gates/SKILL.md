---
name: quality-gates
description: Pre-commit quality checklist — 5 questions that prevent shipping incomplete work
---

# Quality Gates

Before EVERY commit, answer these 5 questions. If ANY answer is "no", STOP and finish the work.

## The 5 Questions

### 1. Did I write negative tests?

Wrong password rejected? Unauthorized access returns 403? Invalid input handled? Tests that only check the happy path are incomplete.

### 2. Did I verify against the LIVE system?

Compilation is not verification. Run actual curl commands, check DB state, use Chrome DevTools. The binary must work, not just build.

### 3. Did I update the companion doc with DETAILS?

Not a one-liner "DONE". Include: commit hashes, test counts, bugs found, DB state changes, deviations from plan.

### 4. Would these tests FAIL without my code change?

A test that passes regardless of your change is decorative. Write tests that exercise the specific behavior you added or fixed.

### 5. Am I moving on because this is TRULY done?

Or because I want to show progress? Thoroughness > speed. Half-done work compounds into tech debt faster than you think.

## Running the Gate

```bash
dstack audit --pre-commit
```

Prints the checklist and asks you to confirm. In strict mode (future), blocks the commit if not confirmed.

## Why This Exists

AI-assisted development accumulates tech debt faster than human development because the feedback loop is shorter. Without quality gates, you ship code that compiles but doesn't work, has no negative tests, and has stale documentation.

This checklist was born from real incidents:
- Mocked tests passing while production migrations failed
- "DONE" companion docs with zero useful detail
- Features that compiled but crashed on first real request
- Tests that passed with or without the code change

## The Doctrine

Implementation -> Unit tests -> Integration tests -> E2E tests -> Live verification -> Doc update -> THEN commit.

ALL of these. EVERY step. EVERY project. Thoroughness > speed. Always.
