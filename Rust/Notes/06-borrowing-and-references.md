# 06 · Borrowing & References

**Source:** `QuickSetup/src/borrowing.rs`

Borrowing lets you access a value **without taking ownership** — so you can read or modify it and
the original owner keeps it. A reference is written `&value` (shared) or `&mut value` (exclusive).

## Why it exists

Moving ownership in and out of every function (note 05) is tedious and often not what you want.
Usually you just need to *look at* or *tweak* a value and hand it back. References are that: a
temporary, non-owning pointer the compiler proves is always valid.

## The borrowing rules

At any given time, for a particular value, you may have **either**:

- **any number of shared references** `&T` (read-only), **or**
- **exactly one mutable reference** `&mut T` (read-write),

**but never both at once.** And every reference must always point to valid data (no dangling).

```rust
let sample_vec = vec![1, 2, 3, 4, 5];
let ref1 = &sample_vec;      // shared borrow
let ref2 = &sample_vec;      // another shared borrow — fine, many readers allowed
// both ref1 and ref2 can read sample_vec simultaneously

let mut sample_vec_two = vec![3, 4, 5, 6, 7];
let ref3 = &mut sample_vec_two;   // exclusive mutable borrow
// let ref4 = &mut sample_vec_two; // ERROR: cannot borrow as mutable more than once
```

This "shared XOR mutable" rule is what prevents data races **at compile time** — you can't have a
writer and any other accessor touching the same data at the same moment.

## `&mut` needs a `mut` binding

You can only take a mutable reference to a variable that is itself declared `mut`:

```rust
let v = vec![1, 2, 3];
// let r = &mut v;     // ERROR: v is not declared mut
let mut w = vec![1, 2, 3];
let r = &mut w;        // ok
```

## References must stay valid (no dangling)

A reference can never outlive the data it points to. The compiler rejects returning a reference to
a local that's about to be dropped. (This is enforced by **lifetimes** — see note 07.)

## Shared vs mutable, summarized

| | `&T` (shared) | `&mut T` (mutable) |
|---|---|---|
| How many at once | many | exactly one |
| Can read | yes | yes |
| Can write | no | yes |
| Coexists with the other kind | no | no |
| Requires `mut` binding | no | yes |

## Recreate this

In `borrowing()`: take two shared `&` references to one vector and use both. Take one `&mut`
reference to a separate `mut` vector; try adding a second `&mut` and confirm the compile error
(comment it out). Note in a comment why "many readers XOR one writer" rules out data races.
