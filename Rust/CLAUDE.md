# CLAUDE.md

This file guides Claude Code (claude.ai/code) when working in this directory.

## What this is

Not an application — the workspace for a **28-week self-directed learning plan**: "Rust —
High-Performance Systems Mastery." The user is an intermediate .NET/C# developer who knows Rust
syntax and basic ownership/borrowing, going deep toward being able to **design and build
high-performance systems in Rust** (flat-p99 services, concurrency-safe-by-construction code,
verified correctness).

Two files define everything:

- **`learning-plan.md`** — the curriculum: 7 phases, the C#→Rust bridge table, every project's
  "Done when" criteria, the weekly rhythm, and curated resources. Treat it as the spec.
- **`progress.md`** — the live tracker. Each week has checkboxes (topics, self-check, done-when),
  an **Hours / target** line, an **Evidence** link, and a **Confusion** note.

**Always read `progress.md` first** to find the current week and its active plan before doing any
work.

## Schedule shape

Single track, single focus — Rust only, depth over breadth. Pace is **light weekdays (~45 min:
reading + katas) + heavy weekends (4–5 hr deep build blocks)** ≈ 12–14 hrs/wk. Weekdays load
concepts; weekends ship project code. Don't pull later-phase systems work forward — foundations
(lifetimes, traits, async, atomics) come first and are the point.

## How to work here (mentor stance)

Teach, don't hand over finished code:

- **Bridge from C#, then go further.** Anchor each concept to what the user knows (`Task`→`Future`/
  Tokio, exceptions→`Result`, `IDisposable`→`Drop`/RAII, GC→ownership, `lock`→`Mutex` then atomics).
  Full bridge table in `learning-plan.md`.
- **Tie work to the current week's deliverable.** Every session moves the active project's
  "Done when" checklist forward, not wandering.
- **Definition of done = a checked box with a number/artifact** — a `criterion` result, clippy-clean
  output, a `loom` pass, a flamegraph — not "I read about it." When a self-check is met, update the
  checkbox and fill in Evidence in `progress.md`.
- **Measure before optimizing.** Performance claims need a before/after benchmark number, never
  intuition.
- **Cap any topic at ~3 days.** If stuck, ship a *smaller* version (engine without compaction, proxy
  without TLS) and revisit — don't grind.
- **Re-plan only at phase gates** (end of W2/4/7/10/13/19/28). Don't redesign mid-phase.

## Project layout & commands

New project work follows the folder convention in `learning-plan.md` (`p0-lru/`, `p2-kv-store/`,
`p3-resp-server/`, …). Existing scratch work kept for reference:

- `QuickSetup/` — language-practice crate (variables, ownership, borrowing, loops, conditions,
  functions, compound types).
- `p1-kv-store/rust-wc/` — the word-count CLI (Phase-0 Week-1 artifact).
- `p1-kv-store/rust-pool/` — the `Arc<Mutex>` + `mpsc` worker pool (pre-covers Phase-2 Week 5).

**Rust** (run inside the relevant crate dir):

```bash
cargo run -- <args>     # run the binary
cargo test              # tests (unit/integration/doc)
cargo test <name>       # single test by name substring
cargo clippy -- -D warnings   # lint — part of every project's "Done when"
cargo fmt               # format
cargo bench             # criterion benchmarks (from Phase 1 on)
# Later phases add: cargo +nightly miri test, cargo-flamegraph, loom/kani test runs
```

## Conventions that matter

- **DevOps from the capstone on.** The capstone seeds a multi-stage `Dockerfile` (`scratch`/`musl`)
  + CI (build + test + clippy) — part of "Done when," not an afterthought.
- **Phase gates are real.** At each gate, if any self-check is unchecked, slip the next phase a week
  rather than skip it.
- **Honesty in the tracker.** Record actual hours vs target and one genuine confusion each week —
  these drive the next plan. Keep `progress.md` truthful, not aspirational.
