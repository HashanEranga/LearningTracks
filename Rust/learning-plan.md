# Rust — High-Performance Systems Mastery

**Profile:** Intermediate .NET/C# dev (async, DI, EF Core, ASP.NET Core) going deep on Rust. Knows Rust syntax + basic ownership/borrowing; building from there to production systems.
**Start:** 2026-06-15 · **Target end:** ~2027-01 (buffer into Feb 2027)
**Pace:** Light weekdays (~45 min/day, theory & katas) + heavy weekends (4–5 hr deep build blocks) ≈ **12–14 hrs/wk**.
**Content length:** 28 weeks · 7 phases · a project every phase, converging on one high-performance capstone.
**Goal:** Become capable of **designing and building high-performance systems in Rust** — the kind that hold a flat p99 under load, are concurrency-safe by construction, and you can *prove* correct.

> **Single track, single focus.** Everything here is Rust. No language is kept "warm" — depth over breadth.

---

## The end state (what "capable of high-performance systems" means)

By the end you can, unprompted:

- Reach for the right ownership / smart-pointer / interior-mutability tool without fighting the borrow checker.
- Write concurrent code and reason about **atomics and memory ordering**, not just `Mutex`.
- Build an **async** service on Tokio and explain the runtime, pinning, and backpressure.
- **Profile** a hot path (flamegraph), kill allocations, exploit cache locality, and drop to `unsafe`/SIMD when the numbers justify it — measured with `criterion`, never guessed.
- Implement a real systems artifact (storage engine / proxy / lock-free structure) and **verify** the concurrency with `loom`/`miri`/`kani`.
- Ship it: Dockerized, benchmarked, instrumented, CI-green.

---

## Guiding principles

- **Definition of done = a checked box with a number or artifact.** Not "I read about it" — a `criterion` result, clippy-clean output, a `loom` pass, a flamegraph. Aspirational checkmarks are forbidden.
- **Lean on C#, then leave it behind.** Anchor each new idea to what you know, then go past it (bridge table below).
- **Measure before you optimize.** Every performance claim needs a before/after number from a benchmark, not intuition.
- **Cap any single topic at ~3 days.** If stuck, ship a *smaller* version (engine without compaction; proxy without TLS) and revisit — never grind.
- **Re-plan only at phase boundaries.** Gates at the end of every phase; if a self-check is unmet, slip the next phase a week rather than skip it.
- **One honest confusion per week** in `progress.md`. The confusion notes drive the next plan.

### C# → Rust bridge (use what you already know)

| You know in C# | Rust |
|---|---|
| `Task` / `async`/`await` | `Future` + `async`/`await` on **Tokio** |
| `IEnumerable<T>` / LINQ | `Iterator` + adapters (`map`/`filter`/`fold`), lazy by default |
| `null` / `Nullable<T>` | `Option<T>` — no null |
| exceptions | `Result<T, E>` + `?` — errors are values |
| interfaces | **traits** (+ bounds, `dyn`, associated types) |
| `IDisposable` / `using` | **RAII** via `Drop` — deterministic, no finalizer queue |
| GC | **ownership / borrowing — no GC, no pauses** |
| `lock (obj) { }` | `Mutex<T>` guard (RAII), and below it: **atomics + `Ordering`** |
| `Span<T>` / `Memory<T>` | `&[T]` / slices / `bytes::Bytes` — zero-copy is the default |
| `unsafe` / `Marshal` / P-Invoke | `unsafe` + raw pointers + **FFI** (`extern "C"`) |

---

## Weekly rhythm (the daily routine)

Fixed cadence so you never decide "what today?". Brain-loading on weekdays, shipping on weekends.

| Day | Time | Job |
|---|---|---|
| **Mon** | ~45 min | Read the week's new concept (the Book / *Rust for Rustaceans*) + take notes |
| **Tue** | ~45 min | Rustlings / Exercism katas drilling that concept |
| **Wed** | ~45 min | Read idiomatic examples + std/crate docs; tiny throwaway experiments |
| **Thu** | ~45 min | Design the weekend increment — sketch the types, write the *failing* test |
| **Fri** | ~45 min | Light: `cargo clippy` + `cargo fmt`, review notes, write the week's one confusion line |
| **Sat** | 4–5 hr | **Deep build** — main project increment |
| **Sun** | 4–5 hr | Continue build → **benchmark, test, update `progress.md`, commit** |

---

## Phase 0 — Language Re-Cement (Weeks 1–2)

**Outcome:** The borrow checker stops being an adversary. You choose `&`/`&mut`/move/clone deliberately and read trait bounds fluently.

| Wk | Topics | Self-check |
|---|---|---|
| 1 | ownership/borrowing/moves *deeply*, **lifetimes** (`'a`, elision, structs holding refs), slices, `String` vs `&str` | Explain in one paragraph *when* a value moves vs borrows vs needs `clone`; a function returning a `&str` into its input compiles |
| 2 | **traits** + bounds + `dyn` vs generics, `enum`/exhaustive `match`, `Result`/`Option`/`?`, **iterators & closures** (`Fn`/`FnMut`/`FnOnce`), `Box`/`Rc`/`Arc`/`RefCell`/`Cell` | A generic `LRU<K, V>` cache (see Project 0) |

### 🔧 Project 0 — Generic LRU cache
A `HashMap` + intrusive doubly-linked list (or `VecDeque` index), generic over `K: Hash + Eq, V`, with `get`/`put`/capacity eviction.
**Done when:** generic over any key/value; unit tests cover eviction order; `clippy` clean; you can explain why the linked-list version needs `Rc<RefCell<..>>` (or an arena/index) and what that costs.

---

## Phase 1 — Idiomatic Rust & Tooling (Weeks 3–4)

**Outcome:** You write Rust that looks like Rust, and you own the measurement toolchain you'll lean on for the rest of the plan.

| Wk | Topics | Self-check |
|---|---|---|
| 3 | modules/crates/workspaces, error design (`thiserror` for libs, `anyhow` for apps), the newtype pattern, `From`/`Into`, builder pattern | A library crate with a clean public API + a typed error enum |
| 4 | `cargo test` (unit/integration/doc), **`criterion`** benchmarking, `proptest` property testing, `clippy` lints deeply, `cargo fmt`, `cargo doc` | A `criterion` bench on Project 0 with a recorded baseline number |

### 🔧 Project 1 — Polish Project 0 into a published-quality crate
Workspace layout, `thiserror` errors, doctests, a `criterion` bench suite, README with usage.
**Done when:** `cargo test` + doctests pass; `criterion` baseline recorded; `clippy -- -D warnings` clean; public API documented.

### Phase 1 Gate (end W4)
- [ ] Comfortable with lifetimes, traits, iterators, error handling, benchmarking? If not → slip Phase 2 a week.

---

## Phase 2 — Fearless Concurrency (Weeks 5–7)

**Outcome:** You build correct multithreaded code and reason *below* the lock — atomics, ordering, and why `Send`/`Sync` are the whole game.

| Wk | Topics | Self-check |
|---|---|---|
| 5 | threads, scoped threads, channels (`mpsc`, `crossbeam-channel`), `Mutex`/`RwLock`, `Arc`, poisoning | Re-derive your `rust-pool` worker pool from scratch, explain each `Arc`/`Mutex` |
| 6 | **`Send`/`Sync`** rules, **atomics + memory ordering** (`Relaxed`/`Acquire`/`Release`/`SeqCst`), `AtomicUsize` counters, `parking_lot` | An atomic counter / spinlock; explain the ordering choices |
| 7 | data-parallelism with **`rayon`**, sharding to cut contention, intro to lock-free thinking, **`loom`** for verifying interleavings | A sharded concurrent map benchmarked vs a single `RwLock` |

### 🔧 Project 2 — Sharded in-memory KV store
`Get`/`Set`/`Delete`, TTL expiry, sharded by key hash across N `RwLock`-guarded maps.
**Done when:** `criterion` shows the sharded version beats a single-lock version under concurrent load (record both numbers); a `loom` test exercises a concurrent invariant; `clippy` clean. *Read* Mara Bos's *Rust Atomics and Locks* alongside this.

### Phase 2 Gate (end W7)

---

## Phase 3 — Async Rust (Weeks 8–10)

**Outcome:** You build an async network service and can explain the runtime under it — not just sprinkle `await`.

| Wk | Topics | Self-check |
|---|---|---|
| 8 | `Future` trait, `async`/`await`, the **Tokio** runtime (tasks, executor, reactor), `spawn`, `JoinHandle`, **function coloring** | A toy `Future` polled by hand; a Tokio "hello" with N spawned tasks |
| 9 | `Pin`/`Unpin` (the why), `select!`, `tokio::sync` (`mpsc`/`oneshot`/`Notify`), cancellation, timeouts | An async pipeline with backpressure that cancels cleanly |
| 10 | async I/O, `AsyncRead`/`AsyncWrite`, **framing/codecs** (`tokio-util`), `Stream`, graceful shutdown | A line-based TCP echo server handling many clients |

### 🔧 Project 3 — Async RESP server (mini-Redis)
A TCP server speaking the **real Redis wire protocol (RESP)**: `GET`/`SET`/`DEL` over Tokio, backed by your Phase-2 sharded store.
**Done when:** the official `redis-cli` can talk to it; load-tested to thousands of concurrent connections with `criterion`/a load tool; graceful shutdown drains in-flight requests; p99 recorded.

### Phase 3 Gate (end W10)

---

## Phase 4 — Performance Engineering (Weeks 11–13)

**Outcome:** You make Rust *fast on purpose* — profile, kill allocations, respect the cache, and drop to `unsafe`/SIMD only when the numbers earn it.

| Wk | Topics | Self-check |
|---|---|---|
| 11 | profiling (`perf`, **`cargo-flamegraph`**, `samply`), reading a flamegraph, memory layout (`#[repr]`, alignment, struct packing), cache locality, SoA vs AoS | Flamegraph a hot path in Project 3 and name the top cost |
| 12 | allocation discipline (`Vec`/`SmallVec`, arenas, `Box` vs stack), **zero-copy** (`bytes::Bytes`, slices), `Cow`, custom allocators (`#[global_allocator]`, `mimalloc`) | A zero-allocation parse hot path proven by a `criterion` alloc count |
| 13 | **`unsafe`** rules + raw pointers + the aliasing model, **`miri`**, **FFI** (`extern "C"`, `bindgen`), `no_std` intro, **SIMD** (`std::simd` / `wide`) | A SIMD or `unsafe` kernel measured ≥2× over the safe version; `miri` clean |

### 🔧 Project 4 — Optimize a real hot path
Take Project 3 (or a CSV/JSON parser) and make it materially faster: flamegraph → zero-copy parsing → fewer allocations → optional SIMD.
**Done when:** before/after `criterion` numbers recorded (target a clear speedup); allocations in the hot loop measured and reduced; any `unsafe` is justified in comments and passes `miri`.

### Phase 4 Gate (end W13)

---

## Phase 5 — Systems Domain Sampler (Weeks 14–19)

**Outcome:** You *feel* the three big high-performance domains by building one focused project in each — then you pick your capstone domain from experience, not a guess.

| Wk | Domain | Build | Self-check |
|---|---|---|---|
| 14–15 | **Databases / storage** | Append-only log → a **Bitcask-style** KV engine: on-disk segments, in-memory index, crash recovery, optional compaction; `mmap` | Survives a process restart with no data loss; throughput recorded |
| 16–17 | **Networking** | A **TCP→L7 proxy / load balancer**: forwards + balances across backends, connection pooling, TLS via `rustls`, graceful drain | Load-balances across 2 backends under load; flat p99 measured |
| 18–19 | **Low-level / compute** | A **lock-free queue** (MPSC/Treiber stack) *or* a **SIMD aggregator**; verify with `loom`/`miri` | Beats the locked version under contention; `loom` passes all interleavings |

> **Capstone fork (end of Phase 5):** you've now built in all three domains. Pick the one that clicked for Phase 6. Default if undecided: extend the **storage engine** toward a distributed KV store — the richest path.

### Phase 5 Gate (end W19)

---

## Phase 6 — High-Performance Capstone (Weeks 20–28)

**Outcome:** One production-grade, verified, benchmarked, deployed high-performance system in your chosen domain — the portfolio centerpiece.

| Wk | Focus |
|---|---|
| 20–21 | Architecture + design doc; if distributed, a **TLA+** spec of the core protocol (replication/membership) — *Learn TLA+* |
| 22–24 | Core build: the data plane / engine, reusing your verified Phase-5 pieces; flat-latency, no-GC hot path |
| 25–26 | Hardening: **`loom`** (interleavings) + **`kani`** (panics/overflow) + `miri`; chaos test (kill a node / inject faults) |
| 27 | Observability + ops: OpenTelemetry traces, Prometheus metrics, Grafana dashboard; multi-stage `Dockerfile` (`scratch`/`musl`); CI |
| 28 | Load test, write the ADRs ("why this design, where the perf comes from"), README + demo, slip/buffer |

### 🏔️ Capstone — pick one
- **Distributed KV store / cache** — Raft (or simpler leader-follower) replication over your storage engine; survives a node kill with no data loss. *(default)*
- **High-performance L7 proxy / API gateway** — your Phase-5 proxy made production-grade: HTTP/2, TLS, rate limiting, flat p99 vs a Go/nginx baseline.
- **Embedded analytics engine** — columnar + SIMD query kernel over a dataset, benchmarked against a naive scan.

**Done when:** correctness verified (`loom`/`kani`/TLA+ where applicable); throughput + **p99/p999 targets hit and recorded**; survives the chaos test; traces + dashboard live; containerized + CI-green; ADRs written.

### Phase 6 Gate (end W28)

---

## Project ladder (each builds on the last)

| # | Project | Phase | Headline skills |
|---|---|---|---|
| 0 | Generic LRU cache | P0 | generics, traits, lifetimes, smart pointers |
| 1 | LRU as a quality crate | P1 | error design, testing, `criterion`, docs |
| 2 | Sharded concurrent KV store | P2 | threads, atomics/ordering, `loom`, sharding |
| 3 | Async RESP server (mini-Redis) | P3 | Tokio, async I/O, framing, backpressure |
| 4 | Hot-path optimization | P4 | flamegraph, zero-copy, `unsafe`/SIMD, `miri` |
| 5a | Bitcask storage engine | P5 | disk I/O, indexing, crash recovery, `mmap` |
| 5b | L7 proxy / load balancer | P5 | networking, `rustls`, pooling, flat p99 |
| 5c | Lock-free queue / SIMD kernel | P5 | atomics, lock-free, `loom`, SIMD |
| 6 | **High-performance capstone** | P6 | verification, distributed/perf, observability, deploy |

---

## Why this is worth it (keep this in view)

- **The biggest names ship Rust at the core.** Cloudflare's **Pingora** replaced nginx and serves over a trillion requests/day. AWS **Firecracker** (the microVM behind Lambda/Fargate) is Rust. **Discord** moved a hot service from Go to Rust specifically to kill GC tail-latency.
- **It's entering the kernel.** Rust has been in the **mainline Linux kernel since 6.1**, and Microsoft is rewriting Windows kernel components in Rust.
- **Governments are mandating the category.** The US ONCD's 2024 report pushed memory-safe languages and named Rust; ~70% of severe CVEs are memory-safety bugs Rust deletes at compile time.
- **The hot databases are Rust:** TiKV, InfluxDB 3.0, Neon, SurrealDB, Qdrant, Materialize, Meilisearch — plus blockchain, HFT, embedded, browsers, and ML infra.
- **It pays, and it's rare.** Rust tops the most-admired-language surveys and sits among the highest-paid SWE specializations. Reasoning about cache lines, allocations, lock-free structures, and async runtimes is **staff/principal-level differentiation** — and it's exactly what "high-performance systems" means on a résumé.

You've already crossed the part most people quit at: ownership clicked, and you built a race-free worker pool the compiler *proved* correct. Everything from here is building real systems and watching your p99 stay flat. That's the fun part — and the part that makes you hard to replace.

---

## Curated resources

- **Language:** *The Rust Programming Language* (the Book) → *Programming Rust* (Blandy/Orendorff) → ***Rust for Rustaceans* (Gjengset)** · Rustlings · Jon Gjengset's "Crust of Rust" videos
- **Concurrency:** **_Rust Atomics and Locks_ (Mara Bos)** — read during Phase 2 · `loom` docs
- **Async:** the **Tokio tutorial** · *Asynchronous Programming in Rust* (the async book) · `mini-redis` reference impl
- **Performance:** *The Rust Performance Book* · `cargo-flamegraph` · *The Rustonomicon* (for `unsafe`) · `miri` docs
- **Systems / databases:** *Designing Data-Intensive Applications* (Kleppmann) · the Bitcask paper · the LSM-tree / RocksDB notes
- **Distributed / verification:** the Raft paper + raft.github.io · MIT 6.5840 labs · *Learn TLA+* (Hillel Wayne) · `kani` docs
- **Production:** *Zero To Production In Rust* (Palmieri) · `tracing` + `tracing-opentelemetry`

---

## Folder structure (suggested)

```
Rust/
├── learning-plan.md            (this file)
├── progress.md                 (weekly tracker — update every Sunday)
├── QuickSetup/                 (language-practice scratch crate — Phase 0 katas)
├── p0-lru/                     (Project 0–1: generic LRU crate)
├── p2-kv-store/                (Project 2: sharded concurrent KV; replaces p1-kv-store)
├── p3-resp-server/             (Project 3: async mini-Redis)
├── p4-perf/                    (Project 4: hot-path optimization)
├── p5-storage-engine/          (Project 5a: Bitcask engine)
├── p5-proxy/                   (Project 5b: L7 proxy)
├── p5-lockfree/                (Project 5c: lock-free / SIMD)
└── capstone/                   (Project 6: the high-performance system)
```

`p1-kv-store/` (rust-wc, rust-pool) is the existing Phase-0 scratch work — keep it for reference; new work lands in the folders above.

---

## Tracking, gates & honesty

Maintain `progress.md`. Each week record: **topics covered · hours (vs ~12–14 target) · self-check done? (Y/N + evidence) · one genuine confusion.**

- **Phase gates** (end of W2/4/7/10/13/19/28): if any self-check is unchecked, slip the next phase a week rather than skip it.
- **Achievability rule:** cap any topic at ~3 days; ship a smaller version if stuck.
- **Holiday buffer:** reserve a slip week around late December.
- **Honesty:** record *actual* hours and a *real* confusion every week — those drive the next plan. Keep the tracker truthful, not aspirational.
