# 03 · Control Flow

**Source:** `QuickSetup/src/conditions.rs`, `QuickSetup/src/loops.rs`

Branching and looping — and the key Rust twist: **most of these are expressions that produce a
value**, not just statements.

## `if` / `else`

A condition must be a real `bool` — Rust has no truthiness, `if 1` is an error.

```rust
if grade >= 50 {
    println!("passed");
} else {
    println!("failed");
}
```

Chain with `else if` for a ladder (first matching branch wins).

### `if` as an expression

`if`/`else` returns a value, so you can bind its result. Both branches must yield the **same type**.

```rust
let rank = if marks > 50 { 'A' } else { 'F' };   // both arms are char
```

The block form `{ if ... { 'A' } else { 'F' } }` does the same — the last expression of a block is
its value (note: no semicolon on that last expression).

## `match`

Pattern matching against a value. The big rule: **`match` must be exhaustive** — every possible
value has to be covered, or it won't compile. `_` is the catch-all "everything else" arm.

```rust
match grade {
    75..=100 => println!("A"),   // ..=  inclusive range (includes 100)
    65..=74  => println!("B"),
    50..=64  => println!("C"),
    35..=49  => println!("D"),
    _        => println!("E"),   // required to make it exhaustive
}
```

Range patterns: `a..=b` is **inclusive** of `b`; `a..b` is **exclusive** of `b`. So `51..100`
matches 51–99, and 100 would fall through to `_`. `match` is also an expression:

```rust
let position = match score {
    0..=50   => 'B',
    51..100  => 'A',
    _        => 'F',
};
```

## Loops

Three kinds:

```rust
loop { ... }                 // infinite until you break
while condition { ... }       // run while the bool holds
for i in vector { ... }       // iterate a collection / range
```

### `loop` returns a value

`break value;` makes the whole `loop` evaluate to that value:

```rust
let counter = loop { break 10; };   // counter == 10
```

### Labeled loops

Label a loop with `'name:` and `break`/`continue` a specific outer loop from inside a nested one:

```rust
'outer: loop {
    loop {
        break 'outer;   // breaks the OUTER loop, not just the inner
    }
}
```

### `for` over ranges and collections

```rust
for i in 0..5 { }            // 0,1,2,3,4  (exclusive end)
for i in (0..5).rev() { }    // 4,3,2,1,0
for i in (0..=10).step_by(2) { }  // 0,2,4,6,8,10
for i in vector { }          // NOTE: this MOVES `vector` (consumes it)
```

⚠️ Ownership note: `for i in vector` takes the vector **by value** and consumes it — you can't use
`vector` afterwards. Use `for i in &vector` to iterate by reference and keep it. (See note 05.)

## Recreate this

In `conditions()`: an `if`/`else`, an `else if` ladder, a `match` with inclusive ranges + `_`,
and bind both an `if`-expression and a `match`-expression to a variable. In `loops()`: a labeled
`loop` with `break 'outer`, a `loop` that returns a value via `break`, a `for` over a vector, and
a `while`.
