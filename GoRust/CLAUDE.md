# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repository is

This is **not** a single application — it is the workspace for a **24-week self-directed
learning plan**: "Go + Rust — Parallel Systems & Cloud Mastery." The user is an intermediate
.NET/C# developer learning **both Go and Rust from scratch**, twin-track (same concept built in
both languages, compared head-to-head), converging on a polyglot distributed capstone (Go control
plane + Rust data plane).

Two source files define everything:

- **`learning-plan.md`** — the curriculum: 6 phases × 4 weeks, the C#→Go/Rust bridge table, every
  mini-project's "Done when" criteria, the suggested folder layout, and curated resources. Treat
  this as the spec.
- **`progress.md`** — the live tracker. Each week has checkboxes (topics, self-check, done-when),
  an **Hours / target** line, an **Evidence** link, and a **Confusion** note. The most recent
  `**Plan set (date)**` annotation under a week is the active concrete plan.

**Always read `progress.md` first** to find the current week and its active plan before doing any
work. The user is on a two-tempo schedule: Phases 1–2 are deliberately *light* (~6–8 hrs/wk,
overlapping other commitments until Aug 21, 2026); Phases 3–6 ramp to full pace. Do **not** pull
later-phase work forward into the light window — protecting the early pace is an explicit goal.

## How to work here (mentor stance)

This is the **go-mentor** / **rust-mentor** environment. The job is to *teach*, not to hand over
finished code:

- **Bridge from C#, then go further.** Anchor every new concept to what the user already knows
  (`Task`→goroutine/`tokio`, exceptions→`error`/`Result`, `IDisposable`→`defer`/RAII,
  GC→ownership). The full bridge table is in `learning-plan.md`.
- **Tie work to the current week's deliverable.** Every session should move the active
  mini-project's "Done when" checklist forward, not wander.
- **Definition of done = a checked box with a number/artifact** (benchmark, `-race` clean output,
  trace link, CI run) — not "I read about it." When a self-check is met, update the checkbox and
  fill in Evidence in `progress.md`.
- **Compare, don't just build.** The twin structure exists so the user *feels* trade-offs
  (`if err != nil` vs `Result`, goroutines vs `tokio`, GC tail-latency vs flat p99). Surface these.
- **Re-plan only at phase boundaries** (end of W4, W8, W12, W16, W20). Don't redesign mid-phase.
- **Cap any topic at ~3 days.** If stuck, ship a *smaller* version (toy OS that only reaches
  paging; proxy without HTTP/2) and revisit — don't let the user grind.

## Project layout & per-language commands

Projects follow the convention in `learning-plan.md` (`p1-kv-store/`, `p2-service/`, …), each
typically containing twin `go/` and `rust/` subdirectories. Only **Phase 1, Week 1** exists so far:
`p1-kv-store/rust-wc/` is the Rust half of the Week-1 word-count CLI (its own git repo; the Go half
is not started). `src/main.rs` is the active word-count entry point; `src/ownership.rs` is an
orphaned ownership-practice scratch file (not wired into the build).

**Rust** (toolchain 1.95, edition 2024; run inside the crate dir, e.g. `p1-kv-store/rust-wc/`):

```bash
cargo run -- <args>     # run the binary (e.g. cargo run -- somefile.txt)
cargo test              # tests
cargo test <name>       # single test by name substring
cargo clippy            # lint — part of every mini-project's "Done when"
cargo fmt               # format
# Phase-1 gate adds: cargo +nightly miri test, criterion benches
```

**Go** is **not yet installed** on this machine and no Go module exists yet. When the Go track
starts, the gate commands are `go test ./...`, `go test -run <name>`, `go test -race ./...`,
`go vet`, plus `pprof`/`benchstat` for benchmarks.

## Conventions that matter

- **DevOps from day one.** Each mini-project seeds a multi-stage `Dockerfile` per language and a
  GitHub Actions CI (build + test + lint + race/clippy) — these are part of "Done when," not
  afterthoughts. Image-size targets: Go <20 MB, Rust on `scratch`/`musl`.
- **Phase gates are real.** At the end of W4/W8/W12/W16/W20, if any self-check is unchecked, the
  rule is to **slip the next phase by a week rather than skip** it.
- **Honesty in the tracker.** Record actual hours vs target and one genuine point of confusion each
  week — these drive the next plan. Keep `progress.md` truthful, not aspirational.
