# Go + Rust — Parallel Systems & Cloud Mastery (24-Week Twin-Track Plan)

**Profile:** Intermediate .NET/C# dev (async, DI, EF Core, ASP.NET Core). Both languages new — *from scratch*.
**Start:** 2026-06-01 · **Target end:** ~2026-12-13 (buffer into Jan 2027)
**Content length:** 24 weeks · 6 phases × 4 weeks · 2 twin projects + 3 individual projects + 1 polyglot capstone
**Goal:** Master **Go for high-demand cloud-native + DevOps** and **Rust for low-level systems (OS/kernel + network infrastructure)** — and learn *when to reach for which*.
**Style:** Twin-track — same concept in both languages, compared head-to-head — converging on **one polyglot distributed capstone** (Go control plane + Rust data plane).
**Threads woven throughout:** correctness & system design (**TLA+** for design, **Loom/Kani** for Rust verification) · **DevOps** (containers + CI from day one, a dedicated cloud/DevOps phase, full GitOps deploy at the end).

---

## ⏱️ Two-tempo schedule (you chose: overlap now)

Your 10–12 hrs/wk is committed to .NET + Angular until **Aug 21, 2026**. So this plan runs in two tempos — the light foundation phases overlap the summer crunch; the heavy systems phases land *after* your other tracks free up:

| Period | Phases | Go+Rust pace | Total weekly load |
|---|---|---|---|
| **Jun 1 → Aug 21** (overlap crunch) | 1–2 (foundations, services) | **~6–8 hrs/wk** | ~16–20 hrs/wk (4 tracks) |
| **Aug 24 → mid-Dec** (.NET/Angular done) | 3–6 (cloud, kernel, network, capstone) | **12–14 hrs/wk** | ~12–14 hrs/wk |

Fortunate sequencing: the hard systems work (kernel, verification, capstone) lands exactly when your calendar opens up.

---

## Guiding principles

- **Both languages stay alive every week.** Phases 1–2 are true twins. Phases 3–5 have a **primary** language and keep the other **warm** (a small weekly exercise) so neither rusts.
- **Bridge from C#, then leave it behind.** Every new concept is anchored to what you already know, then taken further (see bridge table below).
- **Definition of done = a checked deliverable with a number.** Not "I read about it" — "I built it, benchmarked it, and here's the p99 / the trace / the CI run."
- **Compare, don't just build.** The twin structure exists so you *feel* the trade-offs: `if err != nil` vs `Result`, goroutines vs `tokio`, GC tail-latency vs flat p99.
- **Re-plan only at phase boundaries** (end of W4, W8, W12, W16, W20). Don't re-plan mid-phase.

### C# → Go / Rust bridge (lean on what you know)

| You know in C# | Go | Rust |
|---|---|---|
| `Task` / `async`/`await` | goroutine + channel | `async`/`await` + `tokio` |
| `IEnumerable<T>` / LINQ | iterators (`range`-over-func) | `Iterator` + adapters (`map`/`filter`) |
| `null` / `Nullable<T>` | zero values + `, ok` | `Option<T>` (no null) |
| exceptions | `error` values, `errors.Is/As` | `Result<T,E>` + `?` |
| interfaces (explicit) | interfaces (implicit) | traits |
| DI container | explicit wiring / `fx`/`wire` | explicit wiring / constructor fns |
| `IDisposable` / `using` | `defer` | RAII (`Drop`) |
| GC | GC (low-pause) | **ownership/borrowing — no GC** |

---

## Phase 1 — Dual-Language Foundations (Weeks 1–4) · *twin* · light pace

**Outcome:** Productive in both core models. You can reason about Go's simplicity/CSP and survive Rust's ownership/borrow checker without copy-paste.

| Wk | Go track | Rust track | Self-check |
|---|---|---|---|
| 1 | modules/packages, structs, methods, **implicit interfaces**, slices/maps, error values, `log/slog` | **ownership, borrowing, moves**, `&`/`&mut`, `cargo`, `Result`/`Option`, `match` | Same CLI (log-grep / word-count) in both; explain why the Rust version moves vs borrows |
| 2 | goroutines, channels, `select`, `sync` (Mutex/WaitGroup), `context`, `errgroup` | lifetimes, `Box`/`Rc`/`Arc`, `RefCell`, `Mutex`/`RwLock`, `Send`/`Sync`, threads | Worker pool processing N jobs concurrently in both; no data races |
| 3 | generics, interface composition, embedding, functional options | traits + bounds, trait objects (`dyn`), enums, exhaustive `match`, `?`, `thiserror` | A generic LRU cache in both |
| 4 | `go test`, table tests, fuzzing (`testing.F`), **race detector**, `pprof`, `benchstat` | `cargo test`, **`clippy`**, `rustfmt`, `criterion`, `proptest`, **`miri`** | See Mini-Project 1 |

### 🔧 Twin Mini-Project 1 — Concurrent in-memory KV store (both languages)
Get/Set/Delete, TTL expiry, concurrency-safe, with benchmarks. **DevOps seed:** multi-stage `Dockerfile` for each + GitHub Actions CI (build + test + lint + race/clippy) from day one.
**Done when:** Go passes `-race`; Rust passes `clippy` + `miri`; `criterion` vs `benchstat` numbers recorded side-by-side; CI green; images build (<20 MB Go, Rust on `scratch`/`musl`).

---

## Phase 2 — Backend Services & Async I/O (Weeks 5–8) · *twin* · light pace

**Outcome:** Real HTTP + gRPC services in both. You've felt Go's effortless concurrency vs Rust's `async` ceremony first-hand.

| Wk | Go track | Rust track | Self-check |
|---|---|---|---|
| 5 | `net/http` (1.22+ routing), middleware, `context` propagation, graceful shutdown, JSON | `tokio` runtime, `async`/`await`, `Future`, **function coloring**, `Send`/`Sync` bounds, `select!` | TCP echo + basic HTTP handler in both |
| 6 | router/framework (`chi` or `echo`), REST CRUD, validation, error middleware | **`axum` + `tower`** (extractors, layers), REST CRUD | Same tasks REST API in both |
| 7 | `pgx` + **`sqlc`** (type-safe SQL), migrations (`goose`/`atlas`), pooling | **`sqlx`** (compile-time-checked queries), migrations, pooling | Wire tasks API to Postgres in both; note compile-time query checking |
| 8 | **`grpc-go`** + protobuf, interceptors, streaming | **`tonic` + `prost`**, interceptors, streaming | See Mini-Project 2 |

### 🔧 Twin Mini-Project 2 — REST + gRPC service with Postgres (both languages)
**DevOps:** `docker-compose` stack (service + Postgres + OTel Collector + Jaeger); OpenTelemetry tracing on both sides (`otel` Go SDK / `tracing` + `tracing-opentelemetry`).
**Done when:** both sustain 1k concurrent connections; p99 measured + compared; graceful shutdown verified; end-to-end traces visible in Jaeger; `docker compose up` brings the whole stack live.

> **Phase 2 → 3 boundary ≈ Aug 21:** .NET + Angular finish. Ramp Go+Rust to 12–14 hrs/wk.

---

## Phase 3 — Go Cloud-Native & DevOps (Weeks 9–12) · *Go primary, Rust warm* · full pace

**Outcome:** You can ship and operate Go services the cloud-native way, and you own the core DevOps toolchain.

| Wk | Go (primary) | Rust (keep warm) | Self-check |
|---|---|---|---|
| 9 | containers deep (multi-stage, distroless, `trivy` scan), **Kubernetes** core (Deployments, Services, ConfigMaps/Secrets, probes, resources), local cluster (`kind`/`k3d`) | static `musl` build → `scratch` image; cross-compilation | Deploy Phase-2 Go service to `kind` with probes + resource limits |
| 10 | K8s deeper (Ingress, HPA, StatefulSets, RBAC), **Kubernetes Operator** with `controller-runtime` (CRD + reconcile loop), **Helm** | a `clap` CLI that calls the K8s API via `kube-rs` | Operator reconciles a custom resource |
| 11 | **IaC** (Terraform or Pulumi-Go), GitHub Actions advanced (matrix, cache, OIDC, release), **GitOps** (ArgoCD/Flux), secrets | add Prometheus metrics + `tracing` to the Rust service | Service auto-deploys on git push via GitOps |
| 12 | **Observability** (Prometheus + Grafana + OTel Collector + Loki + Jaeger) wired into the project | — | See Project 3 |

### 🔧 Individual Project 3 (Go) — Cloud-Native Operator + Platform
A Kubernetes **operator** managing a CRD (e.g., provisions/operates app instances or a cache cluster), Helm-packaged, **GitOps-deployed via ArgoCD**, with Prometheus metrics + a Grafana dashboard, and e2e tests in CI against `kind`.
**Done when:** operator reconciles the CRD **and self-heals on pod kill**; ArgoCD auto-syncs from git; dashboard is live; CI runs e2e against a `kind` cluster.

---

## Phase 4 — Rust OS / Kernel Level (Weeks 13–16) · *Rust primary, Go warm* · full pace
### Kernel path: hybrid **A (toy OS)** → **C (eBPF/aya)**

**Outcome:** You understand the machine from the metal up *and* can ship kernel-level network/observability programs.

| Wk | Rust (primary) | Go (keep warm) | Self-check |
|---|---|---|---|
| 13 | `#![no_std]`, freestanding binary, the bare-metal model, **`unsafe`** (the rules), **FFI**/C interop, memory layout, custom allocators; `blog_os` toolchain (`bootloader` + QEMU) | — | A `no_std` binary runs in QEMU and prints via serial |
| 14 | **Toy OS core (Path A):** VGA/serial, CPU exceptions + IDT, hardware interrupts (timer/keyboard), **paging + virtual memory**, **heap allocator** (`alloc` enabled) | — | Kernel boots, handles timer + keyboard interrupts, `Vec`/`Box` work on the kernel heap |
| 15 | **eBPF with `aya` (Path C) pt.1:** eBPF model, the **verifier**, maps, userspace loader; an **XDP** packet counter/filter (drop/count by IP/port) | — | XDP program loads on a real kernel and filters live packets |
| 16 | **eBPF pt.2:** a **kprobe/tracepoint** syscall tracer *or* XDP mini load-balancer + userspace Rust controller | a small **Go agent** reads the eBPF maps → Prometheus | See Project 4 |

### 🔧 Individual Project 4 (Rust + a Go agent) — Kernel-level toolkit
Toy OS milestone **plus** an eBPF tool (XDP filter or syscall tracer) with a userspace controller; a Go agent scrapes the eBPF maps and exposes Prometheus metrics.
**Done when:** toy OS boots in QEMU with paging + heap + working interrupts; the eBPF program loads on a real kernel and filters/traces live traffic; the Go agent surfaces the metrics in Grafana.

---

## Phase 5 — Rust Network Infrastructure + Formal Verification (Weeks 17–20) · *Rust primary* · full pace

**Outcome:** You can build a high-performance network data plane and *prove* the correctness of concurrent/distributed code.

| Wk | Rust (primary) | Go (keep warm) | Self-check |
|---|---|---|---|
| 17 | `tokio` internals (tasks, runtime, I/O driver), **zero-copy** with `bytes`, framing/codecs (`tokio-util`), backpressure, custom binary protocol; a TCP proxy | — | A TCP proxy forwards + load-balances across 2 backends |
| 18 | **L7 data-plane proxy:** connection pooling, load balancing, **TLS (`rustls`)**, graceful drain, HTTP/2 | a Go reverse proxy (`net/http/httputil`) | **Measure Go GC tail-latency vs Rust flat p99 under load** — reproduce the Discord story with your own numbers |
| 19 | **TLA+** + PlusCal, the **TLC** model checker, invariants + liveness; spec a replication/consensus protocol (replicated log / Raft safety) | — | TLA+ spec model-checks with no invariant violation |
| 20 | **Verify:** implement the W19 protocol in Rust; **Loom** (all interleavings) + **Kani** (panics/overflow/assertions) | — | See Project 5 |

### 🔧 Individual Project 5 (Rust) — Verified Network Component
A high-throughput L7 proxy *or* a replicated-log node: TLA+-specified, Rust core verified with Loom + Kani, benchmarked under load.
**Done when:** TLA+ spec passes TLC (invariants + liveness); Rust passes Loom (all interleavings) + Kani; sustained throughput target met; **flat p99/p999 demonstrated vs a Go baseline.**

---

## Phase 6 — Polyglot Distributed Capstone + Full DevOps (Weeks 21–24) · *both* · full pace

**Outcome:** One real distributed system using each language where it shines — exactly how industry combines them — shipped with a full GitOps pipeline.

| Wk | Focus | Self-check |
|---|---|---|
| 21 | Architecture + **TLA+ spec** of the core protocol (sharding/replication/membership); define **gRPC contracts** between Go control plane and Rust data plane; repo/workspace setup | Protocol spec model-checks; contract compiles in both langs |
| 22 | **Rust data plane** — storage node / engine + replication hot path (reuse verified pieces from Phase 5), flat-latency, no-GC | A 3-node Rust cluster replicates writes |
| 23 | **Go control plane** as a **Kubernetes operator** — membership, sharding, routing, rebalancing, admin API; OTel/Prometheus/Grafana; wired to Rust nodes via gRPC | Operator manages the cluster lifecycle on K8s |
| 24 | **Integrate + DevOps + chaos + deploy** — containerize both, Helm chart, **ArgoCD GitOps** to a multi-node cluster, load test, **chaos test (kill nodes)**, end-to-end traces, **ADRs** | See Capstone |

### 🏔️ Capstone — Distributed Key-Value Store / Cache
- **Rust** = storage-node data plane (engine + replication hot path, verified, flat p99)
- **Go** = control plane, built as a **Kubernetes operator** (membership, sharding, routing, rebalancing, admin API)
- **TLA+** = the replication protocol, model-checked
- **DevOps** = Docker + Helm + ArgoCD GitOps to a multi-node cluster; Prometheus/Grafana/Jaeger; chaos-tested

**Done when:** survives a node kill with **no data loss + auto-rebalance**; replication protocol TLA+-verified; Rust engine passes Loom; sustained throughput + p99 targets hit; GitOps-deployed on multi-node K8s; full traces + dashboards live; ADRs document "why Go here, why Rust there."

---

## Deliverables summary

| # | Deliverable | Phase / Wk | Type | Key skills |
|---|---|---|---|---|
| 1 | Concurrent KV store | P1 / W4 | twin | both langs core, concurrency, Docker+CI |
| 2 | REST+gRPC+Postgres service | P2 / W8 | twin | net/http+axum, gRPC, sqlc/sqlx, OTel |
| 3 | Cloud-Native Operator | P3 / W12 | Go | K8s, controller-runtime, Helm, GitOps, observability |
| 4 | Kernel-level toolkit | P4 / W16 | Rust(+Go agent) | no_std, toy OS, eBPF/aya (XDP, kprobes) |
| 5 | Verified network component | P5 / W20 | Rust | tokio data plane, rustls, TLA+, Loom, Kani |
| 6 | **Distributed KV capstone** | P6 / W21–24 | polyglot | Go control plane + Rust data plane, TLA+, chaos, full GitOps |

## Topic-to-week index (quick lookup)

- Ownership/borrowing/lifetimes → W1–2 · Go CSP/concurrency → W2
- Traits vs interfaces, generics → W3 · testing/fuzz/`miri`/`criterion` → W4
- `net/http` / `tokio`+`axum` → W5–6 · sqlc/sqlx → W7 · gRPC (grpc-go/tonic) → W8
- Docker/K8s → W9 · Operators (controller-runtime) → W10 · IaC/GitOps → W11 · observability stack → W12
- `no_std`/`unsafe`/FFI → W13 · toy OS (paging/heap/interrupts) → W14 · eBPF/aya (XDP, kprobes) → W15–16
- tokio internals/zero-copy → W17 · L7 proxy/rustls + GC-vs-no-GC bench → W18 · TLA+ → W19 · Loom/Kani → W20
- polyglot architecture + capstone → W21–24

---

## Curated resources

**Go:** *The Go Programming Language* (Donovan & Kernighan) · *100 Go Mistakes* (Harsanyi) · Go by Example · `controller-runtime` + Kubebuilder docs
**Rust:** *The Rust Book* → *Programming Rust* (Blandy/Orendorff) → *Rust for Rustaceans* (Gjengset) · *Zero To Production In Rust* (Palmieri — perfect for Phase 2) · *Rust Atomics and Locks* (Mara Bos — perfect for Phase 5) · Jon Gjengset's "Crust of Rust" videos · the `tokio` tutorial
**OS / kernel:** *Writing an OS in Rust* — os.phil-opp.com (Path A) · OSDev wiki
**eBPF:** the Aya book (aya-rs.dev) · *Learning eBPF* (Liz Rice) · Brendan Gregg's BPF material · Cilium docs (eBPF datapath + **Go** control plane)
**TLA+ / verification:** *Learn TLA+* (learntla.com, Hillel Wayne) · Lamport's TLA+ Video Course · Kani docs · Loom docs
**Distributed systems:** *Designing Data-Intensive Applications* (Kleppmann) · MIT 6.5840 labs · the Raft paper + raft.github.io
**DevOps/cloud:** *Kubernetes Up & Running* · ArgoCD docs · Terraform/Pulumi docs

---

## Folder structure (suggested)

```
GoRust/
├── learning-plan.md            (this file)
├── progress.md                 (weekly tracker)
├── p1-kv-store/                (go/ + rust/  — twin)
├── p2-service/                 (go/ + rust/  — twin)
├── p3-k8s-operator/            (Go — individual)
├── p4-systems/                 (rust-os/ + rust-ebpf/ + go-agent/)
├── p5-verified-proxy/          (Rust + tla/ — individual)
└── capstone-distkv/            (control-plane-go/ + data-plane-rust/ + tla/ + deploy/)
```

---

## Tracking, gates & honesty

Maintain `progress.md`. Each week record: topics covered · hours (vs the tempo target) · self-check done? (Y/N + evidence link) · one thing that confused you.

- **Phase gates** (end of W4, W8, W12, W16, W20): if any self-check is unchecked, slip the next phase by a week rather than skipping.
- **Achievability rule:** cap any single topic at ~3 days. If stuck, ship a *smaller* version (e.g., toy OS that only reaches paging; proxy without HTTP/2) and revisit later.
- **Honest capacity warning:** this is by far the most ambitious of your three tracks. A toy OS and a verified consensus node are each substantial on their own. The overlap window (Jun–Aug, ~16–20 hrs/wk total) is the risk zone — protect Phases 1–2 by keeping them *light*; do **not** pull Phase 3+ work forward into the crunch. If life gets full, stretch to ~30 weeks at ~10 hrs/wk rather than cutting scope.
- **Holiday buffer:** reserve a slip week around late December.
