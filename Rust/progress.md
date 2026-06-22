# Progress Tracker — Rust: High-Performance Systems Mastery

**Start date:** 2026-06-15
**Target end:** ~2027-01 (buffer into Feb 2027)
**Pace:** light weekdays (~45 min) + heavy weekends (4–5 hr blocks) ≈ 12–14 hrs/wk.

Legend: `[ ]` todo · `[~]` in progress · `[x]` done · `[-]` dropped (note why)

Mark each week as you go. **Definition of done = a checked box + a number/artifact** (a `criterion` result, clippy-clean output, a `loom` pass, a flamegraph). Link evidence next to each self-check.

---

## Head start (already done before this plan — carried forward)

- **Syntax & basics** drilled in `QuickSetup/` — variables, compound types, conditions, loops, functions, ownership, borrowing.
- **Word-count CLI** (`p1-kv-store/rust-wc`) — `Option`/`Result`/`?`, `read_to_string`, lines/words/chars cross-checked vs `wc`; clippy clean. *Closes most of Phase 0 Week 1.*
- **Worker pool** (`p1-kv-store/rust-pool`) — `Arc<Mutex<Receiver>>` + `mpsc`, `thread::spawn`/`join`, `drop(tx)` = channel close; race-free by construction. *Pre-covers much of Phase 2 Week 5.*

Net: Phase 0 is largely warm; the gap is **lifetimes, traits/generics depth, iterators/closures** — close those first.

---

## Phase 0 — Language Re-Cement (Weeks 1–2) · Jun 15 → Jun 28

### Week 1 — Ownership/lifetimes deep
- [x] Ownership/borrowing/moves deeply, **lifetimes** (`'a`, elision, structs holding refs), slices, `String` vs `&str` *(ownership/borrowing/moves already solid from rust-wc + QuickSetup; the new work is explicit lifetimes)* — **fn lifetimes done**: `first_word` (elision, no `'a`) + `longest` (explicit `'a`); **struct-held lifetime `Excerpt<'a>` done (2026-06-18)** + deliberately reproduced the **E0597** dangling failure (borrow a `String` from an inner scope, use after drop) and read the three-span error.
- [x] Self-check: move-vs-borrow-vs-clone explainer — **done as working demos + doc-comment in `lifetimes.rs`** (clone = deep heap copy; `&s` borrow = zero-alloc, original stays usable; `#[derive(Copy,Clone)] Point` = bit-copy). Key insight recorded: **borrow/clone are use-site choices; `Copy` is a type-level property** (for trivial stack types, move and copy coincide). A fn returning a `&str` into its input compiles — **✅ `first_word`**
- **Hours**: ~1.5 / ~7 *(today's session est. — correct at Sunday update)* · **Evidence**: `QuickSetup/src/lifetimes.rs` — `first_word` + `longest` + `Excerpt<'a>`; `cargo run` prints expected output, `cargo clippy` 0 warnings *for this file* (pre-existing clippy noise lives in `loops.rs`/`ownership.rs`) · **Confusion**: lifetimes' abstract framing was unclear at first — cleared via book/bookmark analogy + the no-GC bridge (compiler must *prove* a ref doesn't outlive its data). Struct-held lifetimes landed cleanly; the new insight was *reading* the borrow error — rustc blames the borrow-creation line, not the use site (the use site is only the "borrow later used here" reason).

### Week 2 — Traits, generics, iterators
- [~] **traits** + bounds + `dyn` vs generics ✅, `enum`/exhaustive `match` ✅, `Result`/`Option`/`?` ✅, **iterators & closures** (`Fn`/`FnMut`/`FnOnce`) ⬜, `Box`/`Rc`/`Arc`/`RefCell`/`Cell` ⬜ — *traits/generics + enums + Option/Result/? done as runnable katas (`traits_generics.rs`, `enums_match.rs`, `option_result.rs`) with worked Notes 08–10; **iterators/closures + smart pointers still to do**.*
- [ ] **Project 0**: generic `LRU<K, V>` cache — `get`/`put`/capacity eviction
- [ ] Done: generic over any K/V; eviction-order tests; clippy clean; can explain the `Rc<RefCell>` (or arena) cost
- **Hours**: __ / ~7 *(fill at Sunday update)* · **Evidence**: `QuickSetup/src/enums_match.rs` (data-carrying variants, exhaustive `match`, guards, or-patterns/ranges, `if let`/`let else`) + `option_result.rs` (`Option`/`Result` as enums, `map`/`unwrap_or`/`ok_or`/`and_then`/`.ok()`, `?` propagation, `expect`); `cargo run` prints expected output for both, `cargo clippy` 0 warnings *for these files* (pre-existing noise still in `loops.rs`/`ownership.rs`); `Notes/09-enums-and-pattern-matching.md` + `Notes/10-option-result-and-the-question-mark.md`. · **Confusion**: __ *(next: iterators & closures → smart pointers → Project 0 LRU = the Phase-0 gate)*

### Phase 0 Gate (end W2)
- [ ] Lifetimes + traits + iterators comfortable? If not → slip Phase 1 a week.

---

## Phase 1 — Idiomatic Rust & Tooling (Weeks 3–4) · Jun 29 → Jul 12

### Week 3 — Crates, modules, error design
- [ ] modules/crates/workspaces, `thiserror` (libs) vs `anyhow` (apps), newtype, `From`/`Into`, builder
- [ ] Self-check: library crate with clean public API + typed error enum
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 4 — Testing & benchmarking
- [ ] `cargo test` (unit/integration/doc), **`criterion`**, `proptest`, `clippy` deep, `cargo doc`
- [ ] **Project 1**: polish Project 0 into a published-quality crate
- [ ] Done: tests + doctests pass; `criterion` baseline recorded; `clippy -D warnings` clean; API documented
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Phase 1 Gate (end W4)

---

## Phase 2 — Fearless Concurrency (Weeks 5–7) · Jul 13 → Aug 2

### Week 5 — Threads, channels, locks
- [ ] threads, scoped threads, channels (`mpsc`/`crossbeam-channel`), `Mutex`/`RwLock`, `Arc`, poisoning
- [ ] Self-check: re-derive the worker pool from scratch, explain each `Arc`/`Mutex`
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 6 — Atomics & memory ordering
- [ ] **`Send`/`Sync`** rules, **atomics + ordering** (`Relaxed`/`Acquire`/`Release`/`SeqCst`), `parking_lot`
- [ ] Self-check: atomic counter / spinlock; explain the ordering choices
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 7 — Parallelism & sharding → Project 2
- [ ] `rayon`, sharding to cut contention, lock-free intro, **`loom`**
- [ ] **Project 2**: sharded in-memory KV store (Get/Set/Delete + TTL)
- [ ] Done: `criterion` shows sharded beats single-lock under load (both numbers); a `loom` test passes; clippy clean
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Phase 2 Gate (end W7)

---

## Phase 3 — Async Rust (Weeks 8–10) · Aug 3 → Aug 23

### Week 8 — Futures & Tokio runtime
- [ ] `Future`, `async`/`await`, **Tokio** runtime (tasks/executor/reactor), `spawn`, function coloring
- [ ] Self-check: hand-polled toy `Future`; Tokio app with N spawned tasks
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 9 — Pin, select, cancellation
- [ ] `Pin`/`Unpin`, `select!`, `tokio::sync`, cancellation, timeouts
- [ ] Self-check: async pipeline with backpressure that cancels cleanly
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 10 — Async I/O & framing → Project 3
- [ ] `AsyncRead`/`AsyncWrite`, framing/codecs (`tokio-util`), `Stream`, graceful shutdown
- [ ] **Project 3**: async RESP server (mini-Redis) backed by the Phase-2 store
- [ ] Done: `redis-cli` talks to it; load-tested to thousands of conns; graceful drain; p99 recorded
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Phase 3 Gate (end W10)

---

## Phase 4 — Performance Engineering (Weeks 11–13) · Aug 24 → Sep 13

### Week 11 — Profiling & memory layout
- [ ] `perf`, **`cargo-flamegraph`**, reading flamegraphs, `#[repr]`/alignment/packing, cache locality, SoA vs AoS
- [ ] Self-check: flamegraph a hot path in Project 3, name the top cost
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 12 — Allocations & zero-copy
- [ ] `Vec`/`SmallVec`/arenas, `Box` vs stack, **zero-copy** (`bytes`, slices), `Cow`, custom allocators
- [ ] Self-check: zero-allocation parse hot path proven by a `criterion` alloc count
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 13 — unsafe, FFI, SIMD → Project 4
- [ ] **`unsafe`** + raw pointers + aliasing, **`miri`**, **FFI** (`extern "C"`/`bindgen`), `no_std` intro, **SIMD**
- [ ] **Project 4**: optimize a real hot path (flamegraph → zero-copy → fewer allocs → SIMD)
- [ ] Done: before/after `criterion` numbers; allocations reduced; any `unsafe` justified + `miri` clean
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Phase 4 Gate (end W13)

---

## Phase 5 — Systems Domain Sampler (Weeks 14–19) · Sep 14 → Oct 25

### Weeks 14–15 — Databases / storage
- [ ] Append-only log → **Bitcask-style** KV engine: on-disk segments, in-memory index, crash recovery, compaction, `mmap`
- [ ] Self-check: survives a process restart with no data loss; throughput recorded
- **Hours**: __ / ~14 · **Evidence**: · **Confusion**:

### Weeks 16–17 — Networking
- [ ] **TCP→L7 proxy / load balancer**: forwarding, balancing across backends, connection pooling, TLS (`rustls`), graceful drain
- [ ] Self-check: load-balances across 2 backends under load; flat p99 measured
- **Hours**: __ / ~14 · **Evidence**: · **Confusion**:

### Weeks 18–19 — Low-level / compute
- [ ] **Lock-free queue** (MPSC/Treiber) *or* **SIMD aggregator**; verify with `loom`/`miri`
- [ ] Self-check: beats the locked version under contention; `loom` passes all interleavings
- **Hours**: __ / ~14 · **Evidence**: · **Confusion**:

### Phase 5 Gate (end W19) — **pick the capstone domain**
- [ ] All three domain projects done? Capstone domain chosen (default: extend storage engine → distributed KV).

---

## Phase 6 — High-Performance Capstone (Weeks 20–28) · Oct 26 → Dec 27 (+buffer)

### Weeks 20–21 — Architecture + spec
- [ ] Design doc; if distributed, a **TLA+** spec of the core protocol
- **Hours**: __ / ~14 · **Evidence**:

### Weeks 22–24 — Core build
- [ ] Data plane / engine, reusing verified Phase-5 pieces; flat-latency, no-GC hot path
- [ ] Self-check: core feature works end-to-end (e.g. 3-node cluster replicates writes)
- **Hours**: __ / ~14 · **Evidence**:

### Weeks 25–26 — Hardening + verification
- [ ] **`loom`** + **`kani`** + `miri`; chaos test (kill a node / inject faults)
- [ ] Self-check: survives the chaos test with no data loss
- **Hours**: __ / ~14 · **Evidence**:

### Week 27 — Observability + ops
- [ ] OTel traces, Prometheus metrics, Grafana dashboard; multi-stage `Dockerfile` (`scratch`/`musl`); CI
- **Hours**: __ / ~14 · **Evidence**:

### Week 28 — Load test, ADRs, deploy
- [ ] Load test, ADRs, README + demo, buffer/slip
- **Hours**: __ / ~14 · **Deployment / demo**:

### Capstone — done-when checklist
- [ ] Correctness verified (`loom`/`kani`/TLA+ where applicable)
- [ ] Throughput + **p99/p999 targets hit and recorded**
- [ ] Survives the chaos test
- [ ] Traces + dashboard live; containerized + CI-green
- [ ] ADRs written ("why this design, where the perf comes from")
- **Repo**: · **Live demo / numbers**:

### Phase 6 Gate (end W28)

---

## Final retrospective
- **Strongest area:**
- **Weakest area (revisit plan):**
- **Best performance win (your numbers):**
- **Hardest thing (async? atomics? the storage engine?):**
- **Next learning target:**

---

## Running log

| Date | Phase/Wk | Hours | Notes |
|---|---|---|---|
| 2026-06-14 | — | — | Restructured repo to Rust-only; new plan + tracker created. Multi-track state archived at git tag `archive/all-tracks-2026-06-14`. |
| 2026-06-15 | P0 W1 | ~1 | Lifetimes Day 1: `first_word` (elision, no `'a`) + `longest` (explicit `'a`) — both run, clippy clean. Internalized last-expression returns & byte compare. `Excerpt<'a>` (struct holding a ref) parked as a TODO in `lifetimes.rs` for next session. |
| 2026-06-18 | P0 W1 | ~0.5 | Lifetimes Day 2: `Excerpt<'a>` struct done; verified the **E0597** dangling failure (`String` borrowed in an inner scope, used after drop) and dissected the 3-span error — blame at the borrow-creation line, `String` dropped deterministically (no-GC/RAII, C# `using`/`ref struct` bridge), `'static` literals never fire. Lifetimes topic closed. Remaining W1 item (move-vs-borrow-vs-clone) closed same day — see next row. |
| 2026-06-18 | P0 W1 | ~0.3 | Move-vs-borrow-vs-clone self-check closed: working demos in `lifetimes.rs` — `.clone()` (deep heap copy), `&s` borrow (zero-alloc, original stays usable), `#[derive(Copy,Clone)] Point` (bit-copy). Insight recorded: **borrow/clone are use-site choices, `Copy` is a type-level property** (move and copy coincide for trivial stack types). `cargo run` + clippy clean for the file. **Week 1 fully closed.** |
| 2026-06-23 | P0 W2 | __ | Enums + exhaustive `match` and `Option`/`Result`/`?` katas shipped: `enums_match.rs` (unit/tuple/struct-style variants, exhaustive `match`, guards, `\|` or-patterns + `..=` ranges + `_`, `if let`/`let else`) and `option_result.rs` (`Option`/`Result` are enums; `map`/`unwrap_or`/`ok_or`/`and_then`/`.ok()`; `?` propagation across `parse_pair`; `expect` caveat). Both run with expected output; clippy clean for both files. Worked explainers added as `Notes/09-enums-and-pattern-matching.md` + `Notes/10-option-result-and-the-question-mark.md`, indexed in `Notes/README.md`. Remaining W2: iterators & closures → smart pointers → **Project 0 LRU** (Phase-0 gate). |
