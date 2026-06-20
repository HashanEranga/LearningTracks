# 05 · Ownership & Moves

**Source:** `QuickSetup/src/ownership.rs`, plus the move/borrow/clone/Copy demo in `lifetimes.rs`

Ownership is *the* idea that makes Rust memory-safe without a garbage collector. Everything else
(borrowing, lifetimes) is built on it.

## The three rules

1. Every value has a single variable that is its **owner**.
2. There can be **only one owner at a time**.
3. When the owner goes **out of scope**, the value is **dropped** (its memory is freed).

Rule 3 is deterministic: cleanup happens at the closing `}` of the owner's scope — no GC, no
finalizer queue, no pauses.

## Move semantics

For a heap-owning type like `String` or `Vec<T>`, assigning it to another binding **moves** it —
ownership transfers and the original binding becomes invalid:

```rust
let s1 = String::from("world");
let s2 = s1;             // MOVE: s1 is now invalid
// println!("{s1}");    // ERROR: borrow of moved value
println!("{s2}");       // ok
```

Why move instead of copy? `String` owns a heap buffer. Copying the pointer and leaving both `s1`
and `s2` "owning" it would mean a **double free** when both go out of scope. The move makes `s1`
unusable so exactly one owner frees the buffer.

## Functions take and give ownership

Passing a non-`Copy` value to a function **moves** it in. The function now owns it; the caller
can't use it afterward unless the function gives it back via the return value.

```rust
let sample_vec = vec![1, 2, 3, 4, 5];
take_ownership(sample_vec);             // moved IN
// println!("{:?}", sample_vec);        // ERROR: moved

fn take_ownership(vec: Vec<i32>) {       // owns `vec`, drops it at the end
    println!("{:?}", vec);
}

let v = gives_ownership();               // moved OUT (returned)
let v2 = takes_and_gives_ownership(v);   // moved in, then moved back out
```

```rust
fn gives_ownership() -> Vec<i32> {
    let mut sample_vec = Vec::new();
    sample_vec.push(1);
    sample_vec.push(2);
    sample_vec                           // ownership moves to the caller
}
```

This "give it back" dance is exactly why **borrowing** exists (note 06) — usually you just want to
*look at* a value, not take ownership of it.

## The four ways to deal with a value: move / borrow / clone / Copy

When you have a value and want to use it elsewhere, you pick one — and they live at two levels:

| Option | What it does | Cost | Original after? |
|---|---|---|---|
| **move** (default) | transfers ownership | free | unusable |
| **borrow** `&s` | lend a read-only view | zero alloc | still usable |
| **clone** `s.clone()` | deep, independent heap copy | an allocation | still usable |
| **`Copy`** | bit-copy on assignment | trivial | still usable |

```rust
let s = String::from("Hashan");
let cloned_s = s.clone();   // clone: independent heap copy; both usable
let t = &s;                 // borrow: zero-alloc view; s stays usable
```

### `Copy` is a type-level property

Simple stack-only types (`i32`, `bool`, `char`, and structs of only-`Copy` fields with
`#[derive(Copy, Clone)]`) are `Copy`: assignment **copies the bits** and the original stays valid.
There's no heap buffer to share, so there's no double-free risk.

```rust
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

let p1 = Point { x: 1, y: 2 };
let p2 = p1;                 // COPY, not move — p1 is still valid
println!("{} {}", p1.x, p2.x);
```

**Key insight:** borrow vs clone is a **use-site** decision (made per call, based on intent);
`Copy` is a **type-level** property (decided once, on the type). For a trivial `Copy` type, "move"
and "copy" are the same operation.

## Recreate this

In `ownership()`: move a `String` and observe the "moved value" error (comment it out). Write a fn
that takes a `Vec` by value, one that returns a freshly built `Vec`, and one that takes a `Vec` and
returns it modified. In a second demo: clone a `String`, borrow one with `&`, and define a
`#[derive(Copy, Clone)] Point` to show copy-on-assign keeps the original valid.
