# Progress Tracker — Advanced .NET Mastery

**Start date**: 2026-05-29
**Target end date**: 2026-08-21 (12 weeks)

Mark each week as you go. Link evidence (commits, benchmarks, screenshots) next to each self-check.

---

## Phase 1 — Modern C# & Runtime Foundations

### Week 1 — Modern C# language features
- [x] Records, pattern matching, NRT — records + full pattern matching (type/property/relational/`when`) DONE; **NRT** done (`Shape? largest = null`, `FindLargest` returns `Shape?`, null-state analysis caught `CS8600` and forced the `?`)
- [x] Primary constructors, collection expressions, `required`, `file`-scoped types — primary ctors via record positional params (+ `CS9113` class demo earlier); collection expr `Shape[] shapes = [...]`; `required` on `Drawing { Name, Shapes }` (saw `CS9035` when omitting one); `file static class ShapeMath`/`ShapeFormatter`
- [x] Self-check: refactor old class hierarchy to records + exhaustive pattern matching — articulated 3 wins over a legacy `enum`+`switch`+mutable-fields `LegacyShape` (illegal states unrepresentable · compiler-enforced exhaustiveness via `CS8509` · immutability/value-equality); `Classify(Shape)` switch uses property + relational patterns and a `when` guard
- **Hours**: ~8 / 10–12 (this week target ~9 — protected track) *(estimate across May 31 / Jun 2 / Jun 5 — confirm/adjust)*
- **Evidence**: `week01-records-patterns/Program.cs` — `Shape` record hierarchy (abstract base + sealed `Circle`/`Rectangle`/`Square`/`Threpisium`); exhaustive `Area` switch; `Classify` (property/relational/`when`); collection expr; `Drawing` with `required`; `file`-scoped `ShapeMath`/`ShapeFormatter`; `Shape? FindLargest(...)`. Builds (0 errors) & runs; output verified (`Square{Side=80}` is largest at area 6400).
- **Confusion to revisit**: switch-expression exhaustiveness vs the `_` arm — a closed switch with all subtypes covered still emits `CS8509` (the residual null/unknown case), and adding `_ => throw` *silences the missing-subtype compile check you actually want*. Conscious trade-off: no-`_` keeps the build-time "you forgot a type" signal at the cost of one warning. Also: `signal()`-style "two impls of `Area`" DRY smell.
- **Tiny carry-over (non-blocking)**: dedup the two `Area`s — delete the top-level local `Area`, point all callers at `ShapeMath.Area`, and decide the `_` arm deliberately (no-`_` + accept `CS8509`, or `_ => throw new UnreachableException()`; **not** `NotImplementedException`). Optional polish: make the `FindLargest` consumer branch on `is null` so NRT visibly forces the check.
- **Plan set (2026-05-30)**: ~3h records/pattern-matching/NRT · ~3h primary ctors, collection exprs, `required`, `file`-scoped · ~3h self-check refactor (the checkbox that closes W1). Highest-priority track this week.
- **Session log (2026-06-02)**: Covered — record value equality (synthesized `Equals`/`==`/`GetHashCode` + `ToString`), record vs class primary ctor (saw `CS9113` "param unread" on class), abstract base + sealed leaf records as a closed union, type patterns, switch expressions, and **exhaustiveness** (removing `_` → `CS8509` named the unhandled `Threpisium`; `_ => 0` had silently hidden the bug). Next: PascalCase the `Threpisium` params, then property/relational/`when` patterns, then the self-check refactor + NRT + collection exprs/`required`/`file`-scoped.
- **Session log (2026-06-05)**: **Week 1 closed — all 3 boxes.** Built `Classify(Shape)` exercising property patterns (`Circle { Radius: <= 0 }`), relational (`> 100`), and a `when` guard (`Rectangle … Width == Height`) — learned **"pattern if you can, `when` if you must"** (used a guard where a property pattern sufficed for Square, corrected). Hit + fixed a ternary-instead-of-patterns first attempt (empty-string fall-through bug). Task 2 landed all of: collection expr, `required` (`CS9035` when omitted), `file`-scoped helpers, and **NRT** — the keeper lesson was **CS8801** (top-level functions are *local functions inside the synthesized `Main`*, invisible to named types → had to promote `Area` into a type) and **CS8600** (`Shape largest = null` on a non-nullable type → needs `Shape?`). Also a non-compiler logic bug: `FindLargest` never updated `maxArea` (returned last, not largest) — running it surfaced it. Left a tiny non-blocking carry-over (dedup `Area` + decide the `_` arm). **Phase-1 note:** records/patterns/NRT solid; Week 2 is Span/Memory.

### Week 2 — Memory & spans
- [ ] `Span<T>`, `Memory<T>`, `ref struct`, `stackalloc`
- [ ] `ArrayPool<T>`, `IBufferWriter<T>`, `Utf8JsonReader`
- [ ] Self-check: parse 1GB file with zero per-line allocations
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Week 3 — Async deep dive & source generators
- [ ] `ValueTask`, `IAsyncEnumerable`, Channels, `Parallel.ForEachAsync`, cancellation
- [ ] Consume + author a Roslyn source generator
- [ ] Self-check: source generator that emits DTO mappers
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Mini-Project 1 — Log analyzer
- [ ] CLI streams multi-GB log via `IAsyncEnumerable`
- [ ] Span-based parsing, source-generated regex
- [ ] BenchmarkDotNet: zero allocations in hot path
- [ ] 1 GB/min single-threaded; <50 MB working set on 5 GB input
- **Repo link**:
- **Benchmark results**:

### Phase 1 Gate (end of Week 3)
- [ ] All self-checks complete? If not → slip Phase 2 by one week.

---

## Phase 2 — ASP.NET Core + EF Core, Advanced

### Week 4 — Minimal APIs & pipeline
- [ ] Minimal APIs + endpoint filters
- [ ] Custom middleware, model binding, problem details
- [ ] Output caching, rate limiting, response compression, OpenAPI tuning
- [ ] Self-check: Minimal API with versioning, custom binding, rate limiting
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Week 5 — Auth, gRPC, SignalR
- [ ] JWT, OAuth2/OIDC, policy-based authz, requirement handlers
- [ ] Data Protection API
- [ ] gRPC service + client
- [ ] SignalR hubs + streaming
- [ ] Self-check: JWT + role + resource-based authz; gRPC stream
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Week 6 — EF Core advanced
- [ ] Query plans, compiled queries, `AsSplitQuery`, projections
- [ ] Raw SQL, interceptors, migrations strategies
- [ ] Multi-tenancy via global filters, bulk ops
- [ ] Self-check: reduce 2 s query to <50 ms
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Mini-Project 2 — Real-time collaboration API
- [ ] REST (Minimal API) + gRPC + SignalR
- [ ] JWT auth, multi-tenant EF Core
- [ ] Output caching, rate limiting per tenant
- [ ] 100 concurrent SignalR clients sustained
- **Repo link**:
- **Load test results**:

### Phase 2 Gate (end of Week 6)
- [ ] All self-checks complete?

---

## Phase 3 — Architecture, Testing, Observability

### Week 7 — Clean Architecture & DDD
- [ ] Clean Architecture layers
- [ ] DDD aggregates, value objects, domain events
- [ ] CQRS with MediatR, Result + Specification patterns
- [ ] Light Event Sourcing intro
- [ ] Self-check: refactor Project 2 into Clean Arch
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Week 8 — Testing
- [ ] xUnit fixtures/collections/theories
- [ ] `WebApplicationFactory`, Testcontainers (Postgres/Redis)
- [ ] Verify snapshot testing, NetArchTest, Stryker
- [ ] Self-check: >80% line coverage AND >70% mutation score on domain
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Week 9 — Observability & resilience
- [ ] OpenTelemetry (traces, metrics, logs)
- [ ] Serilog with enrichers + sinks
- [ ] Custom metrics with `Meter`/`Counter<T>`
- [ ] Polly v8 `ResiliencePipeline` (retry/circuit-breaker/hedging)
- [ ] Self-check: distributed traces visible; chaos test with Polly
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Mini-Project 3 — Refactor + harden Project 2
- [ ] Clean Architecture + CQRS
- [ ] Integration tests with real Postgres + Redis (Testcontainers)
- [ ] OTel instrumentation end-to-end
- [ ] Polly resilience on outbound HTTP
- [ ] Architecture tests (no domain → infra leaks)
- **Repo link**:
- **Coverage / mutation report**:

### Phase 3 Gate (end of Week 9)
- [ ] All self-checks complete?

---

## Phase 4 — Distributed Systems, Performance, Capstone

### Week 10 — .NET Aspire & messaging
- [ ] .NET Aspire orchestration + dashboard
- [ ] Docker multi-stage builds
- [ ] MassTransit + RabbitMQ / Azure Service Bus
- [ ] Outbox pattern, sagas
- [ ] Redis distributed cache + `HybridCache`
- [ ] Self-check: 3-service Aspire app with messaging + Redis
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Week 11 — Native AOT & profiling
- [ ] Trimming, source generators for JSON/regex, AOT-safe DI
- [ ] `dotnet-trace`, `dotnet-counters`, `dotnet-gcdump`, PerfView
- [ ] GC modes (Workstation vs Server, regions)
- [ ] Self-check: AOT service with <50 ms cold start, <30 MB image
- **Hours**: __ / 10–12
- **Evidence**:
- **Confusion to revisit**:

### Week 12 — Capstone build & deploy
- [ ] All services integrated
- [ ] GitHub Actions CI/CD
- [ ] Deployed publicly (Azure Container Apps / Fly.io / Railway)
- **Hours**: __ / 12–15
- **Deployment URL**:

### Capstone — Mini e-commerce / task-management on Aspire
- [ ] 3+ services, 1 as Native AOT
- [ ] MassTransit messaging + outbox
- [ ] Redis `HybridCache`, Postgres + EF Core, JWT/OIDC auth
- [ ] OpenTelemetry visible in dashboard
- [ ] Polly resilience, integration tests
- [ ] CI/CD pipeline working
- [ ] ADRs in README
- [ ] Sustained 500 req/s on hot read path
- [ ] 70%+ coverage, >65% mutation score
- **Repo link**:
- **Live URL**:

---

## Final retrospective
- **Strongest area**:
- **Weakest area** (revisit plan):
- **Surprises**:
- **Next learning target**:
