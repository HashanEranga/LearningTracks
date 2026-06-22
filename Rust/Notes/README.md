# Rust Notes

Concept-by-concept explanations of everything covered so far, one file per topic. Each note is
self-contained: the idea, the rules, the gotchas, and snippets pulled from the practice code in
`../QuickSetup/`. You should be able to **recreate any exercise from its note alone**.

Pure-Rust explanations — no cross-language analogies.

## Index

| # | Note | Source module | Status |
|---|---|---|---|
| 01 | [Variables & constants](./01-variables-and-constants.md) | `QuickSetup/src/variables.rs` | ✅ |
| 02 | [Compound types](./02-compound-types.md) | `QuickSetup/src/compound_types.rs` | ✅ |
| 03 | [Control flow](./03-control-flow.md) | `QuickSetup/src/conditions.rs`, `loops.rs` | ✅ |
| 04 | [Functions, statements & expressions](./04-functions.md) | `QuickSetup/src/functions_session.rs` | ✅ |
| 05 | [Ownership & moves](./05-ownership-and-moves.md) | `QuickSetup/src/ownership.rs`, `lifetimes.rs` | ✅ |
| 06 | [Borrowing & references](./06-borrowing-and-references.md) | `QuickSetup/src/borrowing.rs` | ✅ |
| 07 | [Lifetimes](./07-lifetimes.md) | `QuickSetup/src/lifetimes.rs` | ✅ |
| 08 | [Traits & generics](./08-traits-and-generics.md) | `QuickSetup/src/traits_generics.rs` | ✅ |
| 09 | [Enums & pattern matching](./09-enums-and-pattern-matching.md) | `QuickSetup/src/enums_match.rs` | ✅ |
| 10 | [Option, Result & `?`](./10-option-result-and-the-question-mark.md) | `QuickSetup/src/option_result.rs` | ✅ |

## Up next (Week 2 remaining)
iterators & closures (`Fn`/`FnMut`/`FnOnce`) → `Box`/`Rc`/`Arc`/`RefCell`/`Cell`
→ **Project 0: generic `LRU<K, V>` cache**.

New notes get added here as each concept is covered.
