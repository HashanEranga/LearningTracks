# LearningTracks

A personal, self-directed learning workspace — curriculum plans, progress trackers, and the
project code built along the way. Each track lives in its own folder with a `learning-plan.md`
(the spec) and a `progress.md` (the live, honest tracker).

## Active track

### 🦀 [Rust — High-Performance Systems Mastery](./Rust)

From "knows the syntax + basic ownership" to **designing and building high-performance systems in
Rust** — flat-p99 services, concurrency-safe-by-construction code, and verified correctness.

- **Plan:** [`Rust/learning-plan.md`](./Rust/learning-plan.md) — 7 phases, 28 weeks, a project every phase
- **Progress:** [`Rust/progress.md`](./Rust/progress.md) — update every Sunday
- **Pace:** light weekdays (~45 min) + heavy weekends (4–5 hr blocks) ≈ 12–14 hrs/wk

**Roadmap at a glance:**

| Phase | Weeks | Theme | Project |
|---|---|---|---|
| 0 | 1–2 | Language re-cement | Generic LRU cache |
| 1 | 3–4 | Idiomatic Rust & tooling | LRU as a quality crate |
| 2 | 5–7 | Fearless concurrency | Sharded concurrent KV store |
| 3 | 8–10 | Async Rust | Async RESP server (mini-Redis) |
| 4 | 11–13 | Performance engineering | Hot-path optimization |
| 5 | 14–19 | Systems domain sampler | Storage engine · proxy · lock-free/SIMD |
| 6 | 20–28 | High-performance capstone | Distributed KV / proxy / analytics engine |

## How it's organized

```
LearningTracks/
├── README.md           (this file)
└── Rust/               (active track — plan + progress + project crates)
```

Additional tracks may be logged here over time, each in its own top-level folder following the same
`learning-plan.md` + `progress.md` convention.

## Conventions

- **Definition of done = a checked box + a number or artifact** (a benchmark result, a clean lint
  run, a passing verification) — never "I read about it."
- **Honest trackers.** Record real hours and a genuine point of confusion each week; those drive the
  next plan.
