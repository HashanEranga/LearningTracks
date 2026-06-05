# Advanced .NET Mastery — 12-Week Plan

**Profile**: Intermediate (ASP.NET Core + EF Core experience)
**Time budget**: 10–12 hrs/week
**Goal**: Full-stack / all-around mastery
**Style**: Project-based — mini-projects per phase + capstone
**Total**: ~140 hours · 4 phases × 3 weeks · 3 mini-projects + 1 capstone

---

## Phase 1 (Weeks 1–3) — Modern C# & Runtime Foundations

| Week | Topics | Hours | Self-check |
|------|--------|-------|------------|
| 1 | Records, pattern matching, NRT, primary constructors, collection expressions, `required` members, `file`-scoped types, init-only | 10–12 | Refactor an old class hierarchy to records + exhaustive pattern matching |
| 2 | `Span<T>`, `Memory<T>`, `ref struct`, `stackalloc`, `ArrayPool<T>`, `IBufferWriter<T>`, `Utf8JsonReader` | 10–12 | Parse a 1GB log file with zero heap allocations per line |
| 3 | Async deep dive: `ValueTask`, `IAsyncEnumerable`, `System.Threading.Channels`, `Parallel.ForEachAsync`, cancellation tokens, source generators (consume + author one) | 10–12 | Write a Roslyn source generator that auto-emits DTO mappers |

### Mini-Project 1 — High-performance log analyzer
CLI tool that streams a multi-GB log file via `IAsyncEnumerable<LogEntry>`, parses with `Span<char>`, uses a source-generated regex, and emits aggregated stats. Benchmark with BenchmarkDotNet.

**Done when**: BenchmarkDotNet shows zero allocations in hot path; tool processes 1 GB/min single-threaded; working set <50 MB on a 5 GB input.

---

## Phase 2 (Weeks 4–6) — ASP.NET Core + EF Core, Advanced

| Week | Topics | Hours | Self-check |
|------|--------|-------|------------|
| 4 | Minimal APIs + endpoint filters, custom middleware, model binding, problem details, output caching, rate limiting, response compression, OpenAPI tuning | 10–12 | Build a Minimal API with versioning, custom binding, and rate limiting |
| 5 | Auth: JWT, OAuth2/OIDC (Duende/IdentityServer or Auth0), policy-based authorization, requirement handlers, Data Protection API. gRPC service + client. SignalR hubs + streaming. | 10–12 | Secure your API with JWT + role + resource-based authz; add a gRPC stream |
| 6 | EF Core advanced: query plans, compiled queries, `AsSplitQuery`, projections, raw SQL + `FromSqlInterpolated`, interceptors, migrations strategies, multi-tenancy via global filters, bulk ops (`ExecuteUpdate`/`ExecuteDelete`) | 10–12 | Reduce a slow query's plan from 2 s to <50 ms; verify with EF Core logging |

### Mini-Project 2 — Real-time collaboration API
A task-board service. REST (Minimal API) + gRPC for sync ops + SignalR for live updates. JWT auth, multi-tenant EF Core with row-level security via global query filters, output caching on read endpoints, rate limiting per tenant.

**Done when**: 3 transport surfaces work (REST/gRPC/SignalR), tenant isolation tested, 100 concurrent SignalR clients sustained.

---

## Phase 3 (Weeks 7–9) — Architecture, Testing, Observability

| Week | Topics | Hours | Self-check |
|------|--------|-------|------------|
| 7 | Clean Architecture, DDD building blocks (aggregates, value objects, domain events), CQRS with MediatR, Result pattern, Specification pattern, light intro to Event Sourcing | 10–12 | Refactor Project 2 into Clean Arch with clear domain layer |
| 8 | Testing: xUnit fixtures/collections/theories, `WebApplicationFactory` integration tests, Testcontainers (Postgres/Redis), Verify snapshot testing, NetArchTest architecture rules, Stryker mutation testing | 10–12 | Achieve >80% line coverage **and** >70% mutation score on domain layer |
| 9 | Observability: OpenTelemetry (traces, metrics, logs), Serilog with enrichers + sinks, structured logging, custom metrics with `Meter`/`Counter<T>`. Resilience with Polly v8 (`ResiliencePipeline`, retry/circuit-breaker/hedging) | 10–12 | View distributed traces in Jaeger/Aspire dashboard; chaos-test with Polly |

### Mini-Project 3 — Refactor + harden Project 2
Clean Architecture, CQRS via MediatR, full integration test suite with Testcontainers (real Postgres + Redis), OTel instrumentation, Polly resilience on outbound HTTP, architecture tests preventing domain → infra leaks.

**Done when**: Mutation score >70% on domain, Jaeger shows end-to-end trace across REST → gRPC → DB → SignalR.

---

## Phase 4 (Weeks 10–12) — Distributed Systems, Performance, Capstone

| Week | Topics | Hours | Self-check |
|------|--------|-------|------------|
| 10 | .NET Aspire (orchestration, service discovery, dashboard). Docker multi-stage builds. Messaging: MassTransit + RabbitMQ (or Azure Service Bus), outbox pattern, sagas. Redis distributed cache + `HybridCache`. | 10–12 | Stand up 3-service Aspire app with messaging + Redis locally |
| 11 | Native AOT: trimming, source generators for JSON/regex, AOT-safe DI. Performance profiling: `dotnet-trace`, `dotnet-counters`, `dotnet-gcdump`, PerfView, Visual Studio profiler. GC modes (Workstation vs Server, regions). | 10–12 | Ship one service as Native AOT; cold start <50 ms, image <30 MB |
| 12 | **Capstone build week**: integrate everything; CI/CD via GitHub Actions; deploy to a free tier (Azure Container Apps / Fly.io / Railway) | 12–15 | Production-grade demo deployed publicly |

### Capstone Project — Mini e-commerce / task-management platform on .NET Aspire
- 3+ services (e.g., Catalog, Orders, Identity) — pick 1 as Native AOT
- Async messaging via MassTransit + outbox pattern
- Redis `HybridCache`, Postgres with EF Core, JWT/OIDC auth
- OpenTelemetry → Aspire dashboard / Grafana
- Polly resilience, integration tests with Testcontainers
- GitHub Actions CI/CD, deployed publicly
- README documenting architecture decisions (ADRs)

**Done when**: All services healthy in production, sustained 500 req/s on the hot read path, traces visible end-to-end, 70%+ test coverage with mutation score >65%.

---

## Tracking & Measurement

Maintain `progress.md` (template in this folder). Each week record:
- [ ] Topics covered (checkbox list from above)
- [ ] Hours spent (target 10–12)
- [ ] Self-check completed? (Y/N + evidence link — commit, screenshot, benchmark output)
- [ ] One thing that confused you (revisit later)

**Mid-phase gate** (end of weeks 3, 6, 9): if any self-check is unchecked, slip the next phase by a week rather than skipping.
**Achievability rule**: cap any single topic at 3 days; if stuck, ship a smaller version and revisit in the next phase.

---

## Curated resources

- **Docs**: Microsoft Learn (.NET 9), `learn.microsoft.com/dotnet/architecture`
- **Books**: *C# in Depth* (Skeet), *Pro .NET Memory Management* (Kokosa), *Concurrency in C# Cookbook* (Cleary)
- **YouTube**: Nick Chapsas, Raw Coding, dotnet team channel
- **OSS to read**: `dotnet/aspnetcore`, `dotnet/efcore`, `MassTransit`, `eShop` reference app

---

## Folder structure (suggested)

```
Dotnet/
├── learning-plan.md          (this file)
├── progress.md               (weekly tracker)
├── project-1-log-analyzer/
├── project-2-collab-api/
├── project-3-clean-refactor/ (same repo as project-2, separate branch ok)
└── capstone-aspire/
```
