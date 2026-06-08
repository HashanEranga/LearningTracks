# DSA Progress Tracker

**Started:** 2026-06-05
**Language:** C# (.NET 8+)
**Spine:** NeetCode 150 → 250, with CSES for DP/graph depth.

## Current status
- **Phase:** 1 — Arrays, Strings & Core Patterns
- **This week's focus:** two-pointer + sliding window (start the NeetCode array set)

## Cadence (set 2026-06-05 by scheduling pass)
DSA runs as a **steady concurrent 5th track** alongside .NET / Angular / Go+Rust — no deferral, work Phases 0→11 in order continuously. User accepts this pushes total load to the top of (slightly over) the ~16–20 hr band.
- **Pace:** **~3–4 hrs/wk (~5–6 problems × ~30 min)**, used as warm-up/edge time since C# is native (recovery, not a competing deep-work block).
- **Order:** strict phase sequence 0→11. When DP (Phase 8) / graphs (Phase 4) arrive they'll need a slightly heavier focus week — absorb by trimming a DSA week elsewhere, not by stalling another track.
- **After Aug 21 cliff** (.NET + Angular end): natural room opens up → can lift DSA toward ~5–6 hrs/wk if desired, but no hard gate.
- **Flip condition:** a hard interview date → compress by jumping to target-company tagged sets + the relevant pattern phases.

## Phase checklist
- [x] Phase 0 — C# toolkit / stdlib — `Snippets.cs` built + runs green (12 sections, .NET 9). Closed 2026-06-08.
- [~] Phase 1 — Arrays, strings, two-pointer, sliding window, prefix sums
- [ ] Phase 2 — Linked lists, stacks, queues, monotonic stack
- [ ] Phase 3 — Trees, BST, recursion
- [ ] Phase 4 — Graphs (BFS/DFS, topo, DSU, Dijkstra, MST)
- [ ] Phase 5 — Searching & sorting, binary search on answer
- [ ] Phase 6 — Backtracking
- [ ] Phase 7 — Greedy & intervals
- [ ] Phase 8 — Dynamic Programming (8.1–8.9)
- [ ] Phase 9 — Advanced DS & string algorithms
- [ ] Phase 10 — Math & bit manipulation
- [ ] Phase 11 — Mocks & spaced repetition

## Log
| Date | Phase | Problems solved | Notes |
|------|-------|-----------------|-------|
| 2026-06-05 | 0 | — | Plan created |
| 2026-06-05 | 0 | — | `phase00-toolkit/Snippets.cs` built (.NET 9), runs green. Key gotchas drilled: max-heap via negate/comparer + no decrease-key, `b.CompareTo(a)` not `b-a`, `int` overflow, `int[][]` vs `int[,]`, LinkedList-as-deque. Next: reproduce-from-memory + LC 347 to close Phase 0. |
| 2026-06-07 | 0 | — | Confident on `Snippets.cs` sections 1–3 (List basics, Dictionary/HashSet, ordered structures). Next: sections 4–12 (heaps onward). |
| 2026-06-07 | 0 | — | Week wrapped. Phase 0 still open. Carryover: sections 4–12 confidence (start §4 Heaps), reproduce heap/freq-map/grid-neighbors from memory, solve LC 347. |
| 2026-06-08 | 0→1 | — | **Phase 0 closed.** Toolkit confident end-to-end. Moving into Phase 1 (arrays/strings/two-pointer/sliding window). |
