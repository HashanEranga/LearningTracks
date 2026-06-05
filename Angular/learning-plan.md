# Advanced Angular Mastery — 12-Week Plan

**Start:** 2026-05-29  
**End:** 2026-08-21  
**Budget:** 5–6 hrs/week (running parallel to active .NET plan)  
**Target stack:** Angular v19+ (signals-first, standalone, zoneless-capable)  
**Capstone:** SaaS Admin Dashboard delivered as a Module-Federation micro-frontend platform

---

## Guiding principles

- **Project-based.** Every week ends in code you can run, not notes you can read.
- **Modern-first.** Default to `inject()`, `signal()`, standalone, functional guards, `@if`/`@for`/`@defer`. Touch the legacy decorator/NgModule path only where it clarifies *why* the modern API exists.
- **One thin slice per week.** ~5–6 hrs = ~1 hr/day or 2–3 longer sessions. If a topic doesn't fit, push the depth, not the schedule.
- **Definition of done = a checked deliverable.** No "I read about it" — only "I built it" or "I broke it on purpose and fixed it."

---

## Phase 1 — Modern Angular Core (Weeks 1–3)

**Outcome:** You can scaffold, structure, and reason about a Signals-first, zoneless Angular app from scratch without copy-pasting boilerplate.

### Week 1 (May 29 – Jun 4) — Standalone + Control Flow
- **Topics:** `bootstrapApplication`, standalone components/directives/pipes, `provideX()` functions, new control flow (`@if` / `@for` / `@switch` / `@defer`), `@let`.
- **Exercises:**
  - Scaffold a v19+ app with `ng new --standalone` (default now).
  - Convert at least one NgModule sample app you find on the web to standalone.
  - Build a "feature toggles" page using `@if` + `@defer (on viewport)`.
- **Done when:** No NgModule in your project, `@defer` is measurably deferring a chunk (verified in Network tab).

### Week 2 (Jun 5 – Jun 11) — Signals Deep Dive
- **Topics:** `signal`, `computed`, `effect`, `untracked`, equality, `signal.update` vs `set`, signal inputs (`input()`, `input.required()`), `output()`, `viewChild()`/`contentChild()` queries, `toSignal`/`toObservable` RxJS interop, linkedSignal/resource API.
- **Exercises:**
  - Reimplement a small RxJS component (search-as-you-type with debounce) using signals + `toSignal`.
  - Build a derived shopping-cart total with `computed`; prove it doesn't recompute when unrelated state changes (log inside `computed`).
- **Done when:** You can explain why `effect()` runs vs. `computed()` recomputes, in your own words, with a code example.

### Week 3 (Jun 12 – Jun 18) — DI + Change Detection → **Mini-Project 1**
- **Topics:** `inject()`, hierarchical injectors, `providedIn: 'root' | 'platform' | 'any'`, `InjectionToken`, multi-providers, route-level providers, `OnPush`, `provideExperimentalZonelessChangeDetection()`, `ChangeDetectorRef` (and why you rarely need it with signals).
- **Mini-Project 1 — Signal Task Board** (4 hrs of the week):
  - Standalone + zoneless + signal-based state, drag-to-reorder columns, persisted to `localStorage` via a token-injected storage service.
  - Acceptance: passes a manual "no `Zone.js` patches" check, all change detection driven by signals, 0 `ChangeDetectorRef` calls.

---

## Phase 2 — Forms, Routing, RxJS, HTTP (Weeks 4–6)

**Outcome:** You can build a complex, validated, multi-step UI backed by typed reactive forms and resilient HTTP.

### Week 4 (Jun 19 – Jun 25) — Reactive Forms (advanced)
- **Topics:** Typed reactive forms, `FormBuilder.nonNullable`, sync + async custom validators, cross-field validation, `ControlValueAccessor` for a custom input, `FormArray` dynamic rows.
- **Exercises:**
  - Build a custom `<app-rating>` star input that implements `ControlValueAccessor`.
  - Build a dynamic "questions" `FormArray` with add/remove and per-row async validation.

### Week 5 (Jun 26 – Jul 2) — Routing (advanced)
- **Topics:** Functional guards (`CanActivateFn`, `CanMatchFn`), `ResolveFn`, lazy-loaded routes (`loadComponent`, `loadChildren`), route-level providers, `withComponentInputBinding`, `withViewTransitions`, preloading strategies, route data + title strategies.
- **Exercises:**
  - Build a 3-level lazy-loaded route tree with a `CanMatchFn` that swaps the loaded component based on a feature flag.
  - Wire `withComponentInputBinding` so route params arrive as signal inputs.

### Week 6 (Jul 3 – Jul 9) — RxJS + HTTP → **Mini-Project 2**
- **Topics:** Higher-order operators (`switchMap`/`concatMap`/`mergeMap`/`exhaustMap`) — when each is correct, `shareReplay` pitfalls, functional `HttpInterceptorFn`, retry with backoff, request cancellation.
- **Mini-Project 2 — Multi-Step Form Wizard** (4 hrs):
  - 4-step wizard with cross-step validation, autosave (debounced), resumable on reload, mock API behind an interceptor that injects auth + 1 retry on 5xx.
  - Acceptance: no leaked subscriptions (verified via a teardown log), wizard state survives refresh.

---

## Phase 3 — State, Performance, Testing (Weeks 7–9)

**Outcome:** You can pick a state strategy on purpose, prove a perf win with numbers, and ship code with tests at the right level.

### Week 7 (Jul 10 – Jul 16) — State Management
- **Topics:** `@ngrx/signals` (Signal Store + entity + withMethods/withComputed), classic `@ngrx/store + effects` (read enough to compare), component-store, when *no* library is the right answer.
- **Exercises:**
  - Build the same "todos with filter" twice: once with Signal Store, once with plain signals + services. Write a 1-page comparison in `progress.md`.

### Week 8 (Jul 17 – Jul 23) — Performance
- **Topics:** Change-detection profiling (Angular DevTools profiler), `@defer` triggers (`on idle`, `on viewport`, `on interaction`, `on hover`, `when`), `*cdkVirtualFor` virtual scrolling, `trackBy` / `@for ... track`, bundle analysis with `esbuild` stats + `source-map-explorer`, image directive (`NgOptimizedImage`), preloading.
- **Exercises:**
  - Take a deliberately slow list (10k rows) and bring first paint under 500ms locally. Record before/after numbers.
  - Add `@defer (on viewport)` to a heavy chart component; verify the chunk loads on scroll only.

### Week 9 (Jul 24 – Jul 30) — Testing → **Mini-Project 3**
- **Topics:** Jest setup for Angular (or modern Karma), component testing patterns with signals, `TestBed` with standalone, Playwright for e2e, network mocking.
- **Mini-Project 3 — Real-Time Analytics Widget** (4 hrs):
  - Signal-store-backed widget receiving simulated WebSocket updates (~50 events/sec), deferred chart, virtual-scrolled event log.
  - Acceptance: ≥1 unit test per public method, 1 Playwright e2e green, profiler shows no change-detection thrash.

---

## Phase 4 — SSR, CDK, Micro-frontends + Capstone (Weeks 10–12)

**Outcome:** You ship a SaaS admin dashboard as a federated host + remotes, with SSR, signal-store, and e2e tests.

### Week 10 (Jul 31 – Aug 6) — SSR + Hydration + i18n + Animations
- **Topics:** `@angular/ssr`, full hydration, **incremental hydration** with `@defer (hydrate on …)`, `TransferState`, request/response tokens, `$localize` i18n, route + enter/leave animations.
- **Exercises:**
  - Add SSR to Mini-Project 3; verify the chart hydrates only when scrolled into view.
  - Localize one route into a second language and switch at runtime.

### Week 11 (Aug 7 – Aug 13) — CDK + Elements + PWA + Module Federation
- **Topics:** CDK overlay/portal, drag-drop, a11y (`FocusTrap`, `LiveAnnouncer`), Angular Elements (custom-element export), `@angular/pwa` service worker, **`@angular-architects/module-federation`** host + remote setup, shared singletons (signals/router).
- **Exercises:**
  - Build a CDK-overlay command palette (cmd+K) with focus trap.
  - Stand up a *throwaway* host + remote pair; load the remote on a route in the host.

### Week 12 (Aug 14 – Aug 21) — **Capstone: SaaS Admin Dashboard (MF Platform)**
- **Architecture:**
  - **Host shell** — auth, role-based routing, layout, command palette, shared signal store.
  - **Remote A — Analytics** — SSR + incremental hydration, virtual-scrolled tables, deferred charts.
  - **Remote B — User Management** — typed reactive forms, dynamic form array, CDK drag-drop role assignment.
- **Cross-cutting:** functional guards (`CanMatchFn` role check), `HttpInterceptorFn` for auth + retry, NgRx Signal Store for shared session, Playwright e2e covering host → remote-A → remote-B navigation, PWA install, Lighthouse ≥ 90 perf on the analytics route.
- **Acceptance checklist:** see `progress.md` Week 12 section.

---

## Deliverables summary

| # | Deliverable | Week | Key skills exercised |
|---|---|---|---|
| 1 | Signal Task Board | 3 | Standalone, signals, zoneless, DI |
| 2 | Multi-Step Form Wizard | 6 | Typed forms, CVA, routing, RxJS, HTTP |
| 3 | Real-Time Analytics Widget | 9 | Signal Store, perf, defer, virtual scroll, testing |
| 4 | **Capstone: SaaS MF Platform** | 10–12 | SSR, hydration, CDK, Module Federation, e2e, PWA |

---

## Topic-to-week index (quick lookup)

- Signals / RxJS interop → W2
- DI / zoneless → W3
- Typed reactive forms / CVA → W4
- Functional guards / lazy / `withComponentInputBinding` → W5
- Higher-order RxJS / functional interceptors → W6
- NgRx Signal Store → W7
- `@defer` / virtual scroll / bundle analysis / `NgOptimizedImage` → W8
- Jest / Playwright → W9
- SSR / incremental hydration / `TransferState` / i18n → W10
- CDK / Elements / PWA / Module Federation → W11
- Capstone integration → W12

---

## How to use this plan

1. Open `progress.md` each Monday; copy that week's checklist into your active session.
2. Time-box: if a topic blows past 6 hrs, drop the lowest-priority exercise (not the deliverable).
3. End every week with one commit on the relevant project repo and one line in `progress.md` under "What surprised me."
4. Re-plan only at phase boundaries (end of W3, W6, W9). Don't re-plan mid-phase.
