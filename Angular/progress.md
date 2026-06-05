# Angular Mastery — Progress Tracker

**Plan:** see `learning-plan.md`  
**Start:** 2026-05-29 · **Target end:** 2026-08-21 · **Budget:** 5–6 hrs/week

Legend: `[ ]` todo · `[~]` in progress · `[x]` done · `[-]` dropped (note why)

---

## Phase 1 — Modern Angular Core

### Week 1 (May 29 – Jun 4) — Standalone + Control Flow
- [x] Scaffold v19+ standalone app (`ng new`) — v21 standalone app `feature-toggles-app`
- [x] Convert one NgModule sample to standalone — built classic `GreetingModule` (declarations/imports/exports + `@Input()`), consumed it from standalone `App`, then flipped `GreetingComponent` back to standalone (deleted the module, `imports` on the component, `@Input()`→`input()`)
- [x] Build a "feature toggles" page with `@if` + `@defer (on viewport)` — `feature-toggles.html`: `@if` toggle + `@defer (on viewport)` with `@placeholder`/`@loading` wrapping `HeavyPanel`
- [x] Verify deferred chunk loads in Network tab — `ng build` tags it `Lazy chunk … heavy-panel`; in DevTools Network the chunk is **absent on load** and **fetches on scroll into viewport** (placeholder → panel swap confirmed)
- **Hours spent:** ~5 / 5–6 (this week target ~5) *(estimate across Jun 3 scaffold + Jun 4 conversion session — confirm/adjust)*
- **What surprised me:** **`signal()` ≠ `input()`.** A `signal()` is private internal state the parent can't bind to — `[heading]="..."` on a `signal` field errors NG8002 *"isn't a known property"*. To receive a parent binding the field must be `input()` (the signal-era `@Input()`). Both read as `heading()` in the template, so they look identical — the difference is *data-flow direction*. Also: a **standalone component can `imports:` an NgModule** (clean interop both ways), and `signal([])` infers `never[]` → must type it `input<string[]>([])`.
- **Plan set (2026-05-30):** ~1.5h scaffold + NgModule→standalone · ~2.5h feature-toggles page with `@if` + `@defer (on viewport)` · ~1h verify deferred chunk in Network tab (acceptance proof — don't skip).
- **Session log (2026-06-04):** Closed Week 1. Did the NgModule→standalone drill *both directions* on the `greeting` component: hand-wrote the old-world `GreetingModule` (learned `declarations` = what you own vs `imports` = other modules' exports vs `exports` = what you expose; `UpperCasePipe` is a standalone pipe so it imports directly), wired it into standalone `App` via `imports: [GreetingModule]`, then converted back to standalone (module file deleted, deps moved onto the component). Hit and resolved the `signal()` vs `input()` binding error (see surprise). `ng build` green; lazy `heavy-panel` chunk confirmed absent-on-load / fetch-on-scroll in Network tab. **Carry to Week 2 (Signals Deep Dive):** the `signal`/`input`/`computed` family is exactly next week's topic — `input()` already met here.

### Week 2 (Jun 5 – Jun 11) — Signals Deep Dive
- [ ] Rebuild a debounced search component using `signal` + `toSignal`
- [ ] Build a cart total with `computed`; log to prove no unnecessary recomputes
- [ ] Write a one-paragraph explainer: `effect()` vs `computed()`
- **Hours spent:** _ / 5–6
- **What surprised me:** 

### Week 3 (Jun 12 – Jun 18) — DI + CD → **Mini-Project 1: Signal Task Board**
- [ ] DI scopes drilled (`root` / `platform` / route-level)
- [ ] Custom `InjectionToken` with multi-provider example
- [ ] Mini-Project 1 shipped (standalone + zoneless + signals + localStorage)
- [ ] Acceptance: 0 `ChangeDetectorRef` calls, no Zone.js patching
- **Hours spent:** _ / 5–6
- **What surprised me:** 

---

## Phase 2 — Forms, Routing, RxJS, HTTP

### Week 4 (Jun 19 – Jun 25) — Reactive Forms
- [ ] Typed `FormGroup` with cross-field validator
- [ ] Custom `<app-rating>` implementing `ControlValueAccessor`
- [ ] Dynamic `FormArray` with async per-row validation
- **Hours spent:** _ / 5–6
- **What surprised me:** 

### Week 5 (Jun 26 – Jul 2) — Routing
- [ ] 3-level lazy route tree with `loadComponent` + `loadChildren`
- [ ] `CanMatchFn` swaps component based on feature flag
- [ ] `withComponentInputBinding` wired to signal inputs
- [ ] Tried `withViewTransitions`
- **Hours spent:** _ / 5–6
- **What surprised me:** 

### Week 6 (Jul 3 – Jul 9) — RxJS + HTTP → **Mini-Project 2: Form Wizard**
- [ ] Wrote one example for each: `switchMap` / `concatMap` / `mergeMap` / `exhaustMap`, with the bug each prevents
- [ ] Functional `HttpInterceptorFn`: auth + retry-on-5xx
- [ ] Mini-Project 2 shipped (4-step wizard, autosave, resumable)
- [ ] Acceptance: no leaked subs, survives refresh
- **Hours spent:** _ / 5–6
- **What surprised me:** 

---

## Phase 3 — State, Performance, Testing

### Week 7 (Jul 10 – Jul 16) — State
- [ ] Built "todos" with `@ngrx/signals` Signal Store
- [ ] Built same with plain signals + services
- [ ] Comparison paragraph below — when would I pick each?

> _Comparison notes here_

- **Hours spent:** _ / 5–6
- **What surprised me:** 

### Week 8 (Jul 17 – Jul 23) — Performance
- [ ] Profiled a 10k-row list; recorded before/after numbers
- [ ] `@defer (on viewport)` on a heavy chart; chunk confirmed lazy
- [ ] Ran bundle analyzer; identified biggest 3 deps
- [ ] Applied `NgOptimizedImage` somewhere real

**Perf numbers (before → after):**

| Scenario | Metric | Before | After |
|---|---|---|---|
| 10k list first paint | ms | _ | _ |
| Chart route bundle | kB | _ | _ |

- **Hours spent:** _ / 5–6
- **What surprised me:** 

### Week 9 (Jul 24 – Jul 30) — Testing → **Mini-Project 3: Analytics Widget**
- [ ] Jest (or Karma) configured for standalone components
- [ ] Playwright project running locally
- [ ] Mini-Project 3 shipped (signal store + WS feed + defer + virtual scroll)
- [ ] Acceptance: ≥1 unit test per public method, 1 e2e green, no CD thrash in profiler
- **Hours spent:** _ / 5–6
- **What surprised me:** 

---

## Phase 4 — SSR, CDK, MF + Capstone

### Week 10 (Jul 31 – Aug 6) — SSR + Hydration + i18n
- [ ] SSR added to Mini-Project 3
- [ ] Incremental hydration via `@defer (hydrate on viewport)` verified
- [ ] `TransferState` removes a duplicate fetch (verified in Network tab)
- [ ] One route localized into a second language, runtime switch works
- **Hours spent:** _ / 5–6
- **What surprised me:** 

### Week 11 (Aug 7 – Aug 13) — CDK + Elements + PWA + MF
- [ ] CDK-overlay command palette (cmd+K) with focus trap
- [ ] One component exported as an Angular Element and loaded in a plain HTML page
- [ ] PWA installable; offline shell works
- [ ] Throwaway host + remote pair: remote loads on a host route
- **Hours spent:** _ / 5–6
- **What surprised me:** 

### Week 12 (Aug 14 – Aug 21) — **Capstone: SaaS Admin (MF Platform)**

**Architecture checklist**
- [ ] Host shell — auth, role-based routing, layout, command palette, shared signal store
- [ ] Remote A: Analytics — SSR + incremental hydration, virtual table, deferred charts
- [ ] Remote B: User Mgmt — typed reactive form, dynamic FormArray, CDK drag-drop

**Cross-cutting**
- [ ] Functional `CanMatchFn` role guard
- [ ] `HttpInterceptorFn` for auth + retry
- [ ] Shared NgRx Signal Store session
- [ ] Playwright e2e: host → remote-A → remote-B happy path
- [ ] PWA installable
- [ ] Lighthouse perf ≥ 90 on analytics route

**Capstone retro (fill in at end):**
- What worked: 
- What I'd change architecturally: 
- Strongest skill gained: 
- Weakest area, plan to fix: 

---

## Running log

| Date | Hours | Notes |
|---|---|---|
| 2026-05-29 | 0 | Plan created |
| 2026-06-04 | ~2 | Week 1 closed — NgModule↔standalone conversion (both directions) on `greeting`; `signal()` vs `input()` lesson; `@defer` lazy chunk verified absent-on-load/fetch-on-scroll. All 4 boxes ticked. |
