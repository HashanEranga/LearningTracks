# Progress Tracker — Go + Rust Systems & Cloud Mastery

**Start date:** 2026-06-01
**Target end:** ~2026-12-13 (buffer into Jan 2027)
**Schedule:** two-tempo — Phases 1–2 light (~6–8 hrs/wk, overlapping .NET/Angular) → Phases 3–6 full (12–14 hrs/wk) after Aug 21.

Mark each week as you go. Link evidence (commits, benchmarks, traces, CI runs) next to each self-check.

---

## Phase 1 — Dual-Language Foundations (twin · light) — Jun 1 → ~Jul 12

### Week 1 — Go basics ‖ Rust ownership
- [x] Go: modules, structs, implicit interfaces, slices/maps, errors, `slog`  *(PARTIAL — genuinely exercised: **modules** (`go mod init`), **errors / `if err != nil`** (`os.ReadFile` err check), **`os.Args`** CLI. NOT exercised by this minimal word-count: **structs/methods, implicit interfaces, slices/maps depth, `slog`** — deferred; some land naturally in Week 2 (worker pool) and later.)*
- [x] Rust: ownership, borrowing, moves, `Result`/`Option`, `match`, `cargo`
- [x] Self-check: same CLI in both; explain move vs borrow  *(DONE — both twins exist; move-vs-borrow explanation captured 2026-05-30. **Evidence**: p1-kv-store/rust-wc/src/main.rs · p1-kv-store/go-wc/main.go)*
- **Hours**: ~4–5 / ~7 (Jun 1–7) · **Evidence**: p1-kv-store/rust-wc/src/main.rs · p1-kv-store/go-wc/main.go · **Confusion**: move vs borrow — internalizing *when* ownership transfers (move) vs *when* a `&`/`&mut` borrow is enough; the flip test ("am I only reading? → borrow") landed but still needs reps against the borrow checker in Week 2's shared-state work.
- **Session (2026-05-30)**: Rust word-count CLI complete — `args.get`/`Option`, `read_to_string`/`Result`, lines/words/chars counts; counts cross-checked against `wc` (6/11/73 on Cargo.toml); clippy clean. **`?` + `main -> Result<(), Box<dyn Error>>` refactor landed**: flattened the nested Ok/Err match into `let contents = read_to_string(path)?;` + `Ok(())`; kept outer Some/None for the usage message. Missing-file now propagates via `?` to the runtime's default printer (`Error: Os { ... NotFound ... }`) instead of a custom message — genuinely idiomatic shape now. Still earned-but-deferred: `eprintln!`/non-zero exit. Evidence: p1-kv-store/rust-wc/src/main.rs. **Verbal half done**: can explain move vs borrow in one sentence — borrow when only reading (the `args.get(1)` → `&String` → `read_to_string` path), move when ownership transfers, clone only for a second independent owned copy; flip test internalized; C# bridge (no GC, compile-time choice) landed. This closes the entire Rust side of Week 1 (code + concept).
- **Session (2026-05-31)**: **Go word-count complete — Week 1 fully closed.** Counts cross-checked Go vs Rust vs `wc` = **6/11/73** (Cargo.toml). `gofmt -l` clean (caught + fixed import ordering / `func main(){`→`func main() {` / missing trailing newline via `gofmt -w`), `go vet` clean, `go build` clean. Key contrast internalized: Rust `Option`/`Result`/`?` vs Go `len(os.Args)` guard + `if err != nil`; **UTF-8 `string` in both Go & Rust** (rune/char count via `utf8.RuneCount` ‖ `.chars().count()`) vs C#'s UTF-16 `string`. `slog` (+ structs/interfaces/maps depth) deferred — not needed for this minimal CLI. Evidence: p1-kv-store/go-wc/main.go.
- **Plan set (2026-05-30)**: Rust-first. **Rust ~4h front-loaded (Jun 1–2)**: ownership, moves, `&`/`&mut`, `Result`/`Option`, `match`, `cargo`. **Go ~3h (Jun 3–5)**: `go mod init`, struct, implicit interfaces, `if err != nil`, `slog`. Shared CLI = **word-count** (`wc`-style: read file arg → lines/words/chars, `Result`/error on missing file). Build Rust half first, then Go half — aiming to close the "same CLI in both" self-check this week. Done-when: word-count runs in both + can explain move vs borrow in one sentence.

### Week 2 — Go concurrency ‖ Rust shared-state
- [ ] Go: goroutines, channels, `select`, `sync`, `context`, `errgroup`  *(NOT STARTED — Go not yet installed on this machine.)*
- [x] Rust: lifetimes, `Box`/`Rc`/`Arc`, `RefCell`, `Mutex`/`RwLock`, `Send`/`Sync`  *(PARTIAL — genuinely exercised: **`Rc`/`RefCell`** (single-threaded, runtime borrow panic on double `borrow_mut`), **`Arc`/`Mutex`** (thread-safe, compile-time `Send`/`Sync`), **`mpsc` channels** both directions, **`thread::spawn`/`move`/`join`**. NOT yet exercised: **lifetimes** (explicit `'a`), **`Box`**, **`RwLock`** — deferred.)*
- [~] Self-check: worker pool in both, no data races  *(RUST HALF DONE — full pool: jobs in via single-producer/multi-consumer `Arc<Mutex<Receiver>>`, 3 workers, results out via multi-producer/single-consumer cloned `Sender`; `cargo clippy` clean; race-free by construction (compiler-enforced, not runtime-checked). GO HALF blocked on Go install.)*
- **Hours**: __ / ~7 · **Evidence**: p1-kv-store/rust-pool/src/multi_worker.rs (worker pool) · p1-kv-store/rust-pool/src/test_drop.rs (channel + `drop`=`CompleteAdding()` demo) · p1-kv-store/rust-wc/src/ShareMutable.rs (`Rc`/`RefCell`) · **Confusion**:
- **Session (2026-06-08)**: **Rust worker pool complete — Week-2 Rust self-check half met.** Evolved `rust-pool` from a single-thread `Arc<Mutex<i32>>` counter into a real N-worker pool. Key concepts landed: **`drop(tx)` ≈ C#'s `BlockingCollection.CompleteAdding()`** — channel closes when the *last* `Sender` drops; felt the forever-hang when `drop` omitted. **`mpsc` shape asymmetry internalized**: jobs side is single-producer/**multi**-consumer so the non-`Clone` `Receiver` needed `Arc<Mutex<Receiver>>` (workers take turns locking `recv()`); results side is **multi**-producer/single-consumer so each worker just `.clone()`s the `Sender` — no mutex (that's literally what `mpsc` is built for). **Data-race freedom is compile-time** (`send` *moves* the value → one owner → race unrepresentable) vs C#'s runtime `-race`-style trust. Two traps felt, not just read: (1) lock held *during* the work → fully sequential (correct version scopes the guard to just `recv()` via the `.lock().unwrap().recv()` temporary-drop trick); (2) results-collection loop doubles as the join barrier — `for ... in result_rx` blocks until all senders drop, so explicit `join()` was unnecessary *here*. C# bridge throughout: `BlockingCollection`→`mpsc`, `lock(){}`→`MutexGuard` RAII, `Task`/`ThreadPool`→`thread::spawn`. **Note**: stuck with std `mpsc`+`Arc<Mutex<Receiver>>`; `crossbeam-channel` (clonable `Receiver`) shown as the ecosystem alternative but not adopted — deferred.

### Week 3 — Generics/interfaces ‖ traits/enums
- [ ] Go: generics, interface composition, embedding, functional options
- [ ] Rust: traits + bounds, `dyn`, enums, `match`, `?`, `thiserror`
- [ ] Self-check: generic LRU cache in both
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 4 — Tooling & testing → Mini-Project 1
- [ ] Go: `go test`, fuzzing, `-race`, `pprof`, `benchstat`
- [ ] Rust: `cargo test`, `clippy`, `criterion`, `proptest`, `miri`
- [ ] **Mini-Project 1**: concurrent KV store (both); Docker + CI
- [ ] Done: `-race` clean; `clippy`+`miri` clean; bench numbers recorded; CI green; images <20MB
- **Repo**: · **Benchmark (Go vs Rust)**:

### Phase 1 Gate (end W4)
- [ ] All self-checks complete? If not → slip Phase 2 a week.

---

## Phase 2 — Backend Services & Async (twin · light) — ~Jul 13 → Aug 23

### Week 5 — `net/http` ‖ `tokio`
- [ ] Go: `net/http` routing, middleware, context, graceful shutdown
- [ ] Rust: `tokio`, `async`/`await`, futures, function coloring, `select!`
- [ ] Self-check: TCP echo + HTTP handler in both
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 6 — frameworks ‖ axum
- [ ] Go: `chi`/`echo`, REST CRUD, validation, error middleware
- [ ] Rust: `axum` + `tower`, REST CRUD
- [ ] Self-check: same tasks API in both
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 7 — persistence (sqlc ‖ sqlx)
- [ ] Go: `pgx` + `sqlc`, migrations, pooling
- [ ] Rust: `sqlx` compile-time-checked, migrations, pooling
- [ ] Self-check: tasks API on Postgres in both
- **Hours**: __ / ~7 · **Evidence**: · **Confusion**:

### Week 8 — gRPC → Mini-Project 2
- [ ] Go: `grpc-go` + protobuf, streaming
- [ ] Rust: `tonic` + `prost`, streaming
- [ ] **Mini-Project 2**: REST+gRPC+Postgres (both); compose + OTel + Jaeger
- [ ] Done: 1k concurrent conns; p99 compared; traces in Jaeger; `compose up` works
- **Repo**: · **p99 (Go vs Rust)**:

### Phase 2 Gate (end W8 ≈ Aug 21 — .NET/Angular finish, ramp to full pace)
- [ ] All self-checks complete?

---

## Phase 3 — Go Cloud-Native & DevOps (Go primary · full) — Aug 24 → Sep 20

### Week 9 — containers + K8s core
- [ ] Multi-stage/distroless, `trivy`; K8s Deployments/Services/Config/Secrets/probes; `kind`/`k3d`
- [ ] Rust-warm: `musl` static → `scratch` image
- [ ] Self-check: deploy Phase-2 Go service to `kind`
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 10 — Operators + Helm
- [ ] Ingress, HPA, StatefulSets, RBAC; `controller-runtime` operator (CRD + reconcile); Helm
- [ ] Rust-warm: `kube-rs` CLI
- [ ] Self-check: operator reconciles a CRD
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 11 — IaC + GitOps
- [ ] Terraform/Pulumi-Go; GH Actions advanced; ArgoCD/Flux GitOps; secrets
- [ ] Rust-warm: Prometheus metrics + `tracing` on Rust service
- [ ] Self-check: GitOps auto-deploy on push
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 12 — observability → Project 3
- [ ] Prometheus + Grafana + OTel Collector + Loki + Jaeger
- [ ] **Project 3 (Go)**: operator + Helm + ArgoCD + metrics + dashboard + e2e CI
- [ ] Done: reconciles + self-heals on pod kill; ArgoCD syncs; dashboard live; e2e on `kind`
- **Repo**: · **Dashboard link**:

### Phase 3 Gate (end W12)
- [ ] All self-checks complete?

---

## Phase 4 — Rust OS/Kernel (Rust primary · full) — Sep 21 → Oct 18 · path A+C

### Week 13 — no_std / unsafe / FFI
- [ ] `#![no_std]`, freestanding binary, `unsafe` rules, FFI, allocators; `blog_os` toolchain
- [ ] Self-check: `no_std` binary prints via serial in QEMU
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 14 — toy OS core (Path A)
- [ ] VGA/serial, exceptions + IDT, timer/keyboard interrupts, paging, heap allocator
- [ ] Self-check: kernel handles interrupts; `Vec`/`Box` work on kernel heap
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 15 — eBPF/aya pt.1 (Path C)
- [ ] eBPF model, verifier, maps, userspace loader; XDP packet counter/filter
- [ ] Self-check: XDP program filters live packets
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 16 — eBPF pt.2 → Project 4
- [ ] kprobe/tracepoint tracer or XDP LB + Rust controller
- [ ] Go-warm: Go agent reads eBPF maps → Prometheus
- [ ] **Project 4**: toy OS milestone + eBPF tool + Go agent
- [ ] Done: OS boots w/ paging+heap+interrupts; eBPF loads on real kernel; agent metrics in Grafana
- **Repo**:

### Phase 4 Gate (end W16)
- [ ] All self-checks complete?

---

## Phase 5 — Rust Network Infra + Verification (Rust primary · full) — Oct 19 → Nov 15

### Week 17 — tokio internals / data plane
- [ ] tokio internals, zero-copy (`bytes`), codecs, backpressure, custom protocol; TCP proxy
- [ ] Self-check: proxy load-balances across 2 backends
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 18 — L7 proxy + the GC bench
- [ ] Connection pooling, LB, TLS (`rustls`), graceful drain, HTTP/2; Go `httputil` proxy
- [ ] Self-check: **Go GC tail-latency vs Rust flat p99** measured under load
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 19 — TLA+
- [ ] TLA+/PlusCal, TLC, invariants + liveness; spec a replication/consensus protocol
- [ ] Self-check: spec model-checks, no invariant violation
- **Hours**: __ / 12–14 · **Evidence**: · **Confusion**:

### Week 20 — Loom/Kani → Project 5
- [ ] Implement W19 protocol in Rust; Loom (interleavings) + Kani
- [ ] **Project 5 (Rust)**: verified L7 proxy or replicated-log node
- [ ] Done: TLC passes; Loom + Kani pass; throughput target; flat p99 vs Go baseline
- **Repo**: · **p99/p999**:

### Phase 5 Gate (end W20)
- [ ] All self-checks complete?

---

## Phase 6 — Polyglot Capstone + Full DevOps (both · full) — Nov 16 → Dec 13 (+buffer)

### Week 21 — architecture + TLA+ + contracts
- [ ] TLA+ spec of core protocol; gRPC contracts; workspace setup
- **Hours**: __ / 12–14 · **Evidence**:

### Week 22 — Rust data plane
- [ ] Storage node/engine + replication hot path (reuse Phase-5 verified code)
- [ ] Self-check: 3-node Rust cluster replicates writes
- **Hours**: __ / 12–14 · **Evidence**:

### Week 23 — Go control plane (operator)
- [ ] Membership, sharding, routing, rebalancing, admin API; OTel/Prom/Grafana; gRPC to Rust nodes
- [ ] Self-check: operator manages cluster lifecycle on K8s
- **Hours**: __ / 12–14 · **Evidence**:

### Week 24 — integrate + chaos + deploy
- [ ] Containerize both, Helm, ArgoCD GitOps, load test, chaos (kill nodes), traces, ADRs
- **Hours**: __ / 12–15 · **Deployment URL**:

### Capstone — Distributed KV store / cache
- [ ] Rust data-plane nodes (verified, flat p99)
- [ ] Go control plane as K8s operator
- [ ] TLA+-verified replication
- [ ] Survives node kill: no data loss + auto-rebalance
- [ ] Rust engine passes Loom
- [ ] Throughput + p99 targets hit
- [ ] GitOps-deployed on multi-node K8s; traces + dashboards live
- [ ] ADRs ("why Go here, why Rust there")
- **Repo**: · **Live cluster / demo**:

### Phase 6 Gate (end W24)
- [ ] Capstone done-when checklist complete?

---

## Final retrospective
- **Stronger language (and why)**:
- **Go: cloud/DevOps takeaway**:
- **Rust: where it clearly beat Go (your numbers)**:
- **Hardest thing (kernel? async Rust? TLA+?)**:
- **Next learning target**:
