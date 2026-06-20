# 01 · Variables & Constants

**Source:** `QuickSetup/src/variables.rs`

## The idea

`let` introduces a binding. By default it is **immutable** — once bound, the value can't change.

```rust
let x = 10;        // immutable: `x = 11;` would be a compile error
let mut y = 10;    // mutable: `y = 11;` is allowed
```

Immutable-by-default is a core Rust stance: you opt *into* mutation with `mut`, so anything not
marked `mut` is guaranteed not to change underneath you.

## Type inference

You rarely write the type — the compiler infers it from the value. `let x = 10;` infers `i32`
(the default integer type). You annotate only when it's ambiguous or you want a specific type:

```rust
let a: u8 = 255;       // explicit type
let b = 10u64;         // type suffix on the literal
```

## Shadowing vs mutation

You can re-`let` the same name. This is **shadowing** — a brand-new binding that reuses the name,
and it can even change the type. Different from `mut`, which reuses the same binding/type.

```rust
let s = "42";              // &str
let s = s.len();           // now usize — shadowed, type changed
let mut n = 0;             // mut: same binding, same type, value can change
n += 1;
```

## Constants

`const` is a compile-time constant: always immutable, type **must** be annotated, and it can be
declared at any scope including module top-level. Naming convention is `SCREAMING_SNAKE_CASE`.

```rust
const MAX_USERS: u32 = 10_000;   // `_` is a digit separator, purely cosmetic
```

`const` differs from an immutable `let`: a `const` is inlined at every use site and has no fixed
memory address; a `let` is a runtime binding.

## Recreate this

In `variables()`: bind an immutable `let`, a `mut` variable and mutate it, shadow a binding to
change its type, and declare a `const`. Print each to confirm.
