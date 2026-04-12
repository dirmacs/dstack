# dstack Harness Patterns

Six reusable team topologies for AI-driven multi-agent work.
Each pattern names the shape of coordination, when to use it,
and a minimal dstack config skeleton.

---

## 1. Pipeline

**Shape:** Sequential chain — agent A output feeds agent B, then agent C.

**When:** Each step depends on the previous result. Data transformation,
staged analysis, generate → review → refine loops.

**Config sketch:**
```toml
[[agent]]
name = "researcher"
task = "gather requirements"
next = "writer"

[[agent]]
name = "writer"
task = "draft from researcher output"
next = "reviewer"

[[agent]]
name = "reviewer"
task = "critique and finalize writer output"
```

---

## 2. Fan-out

**Shape:** One coordinator spawns N workers in parallel, collects results.

**When:** Tasks are independent and can run concurrently — parallel code
analysis, multi-language translation, batch PR reviews.

**Config sketch:**
```toml
[[agent]]
name = "coordinator"
task = "split work into subtasks, spawn workers, merge results"
fan_out_workers = ["worker-a", "worker-b", "worker-c"]
```

---

## 3. Expert Pool

**Shape:** Router dispatches each request to the most relevant specialist.

**When:** Domain diversity in a single queue — some requests need a
security expert, others a frontend specialist, others a DB optimizer.

**Config sketch:**
```toml
[[agent]]
name = "router"
task = "classify input and delegate to the right specialist"

[[agent]]
name = "security-expert"
domain = "security, auth, cryptography"

[[agent]]
name = "frontend-expert"
domain = "UI, CSS, React, accessibility"

[[agent]]
name = "db-expert"
domain = "SQL, migrations, query optimization"
```

---

## 4. Supervisor

**Shape:** Orchestrator + worker loop. Orchestrator plans and validates;
worker executes. Orchestrator re-plans if the worker stalls or errs.

**When:** Tasks require adaptive planning — the plan changes based on
execution feedback. Debugging sessions, iterative implementation.

**Config sketch:**
```toml
[[agent]]
name = "supervisor"
task = "create a plan, delegate to worker, validate result, re-plan if needed"
supervises = ["worker"]
max_replanning_iterations = 3

[[agent]]
name = "worker"
task = "execute the current plan step"
```

---

## 5. Hierarchical

**Shape:** Tree of orchestrators — a root coordinator delegates to
sub-coordinators, each of which manages their own worker pool.

**When:** Very large tasks that must be decomposed into independently
manageable sub-problems. Monorepo refactors, multi-service migrations.

**Config sketch:**
```toml
[[agent]]
name = "root"
task = "break the project into sub-domains, delegate to sub-coordinators"
delegates_to = ["frontend-coord", "backend-coord", "infra-coord"]

[[agent]]
name = "frontend-coord"
task = "coordinate all frontend work"
workers = ["ui-agent", "test-agent"]

[[agent]]
name = "backend-coord"
task = "coordinate all backend work"
workers = ["api-agent", "db-agent"]
```

---

## 6. Perpetual Iteration

**Shape:** Three parallel loops on co-prime intervals (3 / 7 / 11 minutes).
Each loop picks work from a shared task queue, commits, and reschedules.
The system continues autonomously until all tasks are complete or the
session ends.

**When:** Long-horizon projects where you want continuous progress without
sitting at the keyboard. The loops fire independently, spreading work
across time and preventing any single concern from starving.

**Loop roles:**
| Loop | Cadence | Concern | Fallback if idle |
|---|---|---|---|
| Execution | 3 min | Main tasks — pick next unblocked task, do one commit's worth | Add tests or write intel |
| Quality | 7 min | Test coverage — rotate through modules, add 3-5 tests | Intel update |
| Intel | 11 min | Docs, memory, architecture — dead code, drift, eruka writes | Cross-repo scan |

Why co-prime intervals? LCM(3, 7, 11) = 231 minutes — the three loops
align fully only once every ~4 hours, giving each concern independent
cadence rather than synchronised bursts.

**Config sketch:**
```toml
[perpetual_iteration]
enabled = true

[[perpetual_iteration.loop]]
name = "execution"
interval_secs = 180
task = """
Check task list for the next unblocked pending task.
Work on it. Produce one commit. Reschedule.
If no tasks remain, expand test coverage.
"""

[[perpetual_iteration.loop]]
name = "quality"
interval_secs = 420
task = """
Pick the module in the workspace with the lowest test density.
Add 3-5 meaningful tests covering happy path, error cases, and edges.
Commit. Reschedule.
"""

[[perpetual_iteration.loop]]
name = "intel"
interval_secs = 660
task = """
Rotate through: dead code hunt → coupling audit → doc drift →
memory refresh → cross-repo scan → eruka write → session log.
Do one option per firing. Commit output. Reschedule.
"""

[perpetual_iteration.halt_conditions]
# Stop if N consecutive execution-loop iterations fail
consecutive_failures = 3
# Optional hard cap on total iterations (0 = unlimited)
max_total_iterations = 0
```

**Anti-idle guarantee:** every loop firing MUST produce a commit,
subagent spawn, or knowledge write. Reporting "nothing to do" is
a loop failure — the loops walk a fallback hierarchy before giving up.

---

## Choosing a pattern

| Scenario | Pattern |
|---|---|
| Build → test → deploy CI | Pipeline |
| Parallel test runners | Fan-out |
| Mixed-domain support queue | Expert Pool |
| Iterative debugging / red-green cycles | Supervisor |
| Monorepo-wide migration | Hierarchical |
| Long-horizon improvement project (overnight) | Perpetual Iteration |

Patterns can be composed. A Perpetual Iteration loop can internally
spawn Fan-out agents for heavy parallel steps; a Hierarchical tree
can put a Supervisor at each leaf.
