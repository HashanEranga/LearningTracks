# 07 · Lifetimes

**Source:** `QuickSetup/src/lifetimes.rs`

A lifetime is the span during which a reference is valid. Lifetimes are how the compiler **proves**
a reference never outlives the data it points to — the no-dangling guarantee from note 06, made
explicit. They are checked at compile time and have zero runtime cost.

You usually don't write them: **elision** infers them. You write `'a` only when inference is
ambiguous, or when a struct stores a reference.

## Exercise 1 — `first_word`: elision, no `'a` needed

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];     // a slice borrowed FROM the input
        }
    }
    s
}
```

One input reference → the elision rules tie the output's lifetime to that single input → you don't
have to annotate anything. The returned `&str` borrows from `s`, which is exactly what we want.

## Exercise 2 — `longest`: ambiguous, so `'a` is required

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

Two input references and the result could come from **either** of them. The compiler can't infer
which, so you declare a shared lifetime `'a`: "the result is valid for the **shorter** of the two
inputs' lifetimes." The annotation doesn't change how long anything lives — it just states the
relationship so the compiler can verify callers.

## Exercise 3 — `Excerpt<'a>`: a struct holding a reference

```rust
struct Excerpt<'a> {
    part: &'a str,
}
```

**There is no lifetime elision for struct fields.** If a struct stores a reference, you *must*
annotate it: `'a` says "an `Excerpt` may not outlive the `str` it points into." The compiler then
refuses to let an `Excerpt` survive past its borrowed source.

## The failure to internalize: E0597

Reproduced deliberately, then removed: borrow a `String` created in an inner scope, let the scope
close (dropping the `String`), then use the `Excerpt`:

```text
error[E0597]: `novel` does not live long enough
   |  let excerpt = Excerpt { part: &novel };
   |                                ^^^^^^ borrowed value does not live long enough
   |  }
   |  - `novel` dropped here while still borrowed
   |  println!("{}", excerpt.part);
   |                 ------------- borrow later used here
```

Three things to read off it:
1. rustc blames the **borrow-creation** line (where `&novel` is taken), not where you used it.
2. "dropped here while still borrowed" points at the **closing brace** — `String` is freed
   deterministically at scope end (no GC keeping it alive).
3. "borrow later used here" points at the final use — that's the *reason* the borrow needed to
   live longer than its source did.

A string **literal** is `&'static str` (lives for the whole program), so borrowing from a literal
never triggers this.

## Elision rules (the shorthand the compiler applies)

1. Each input reference gets its own lifetime parameter.
2. If there is exactly **one** input lifetime, it's assigned to **all** outputs (→ `first_word`).
3. (For methods) if one input is `&self`/`&mut self`, its lifetime goes to all outputs.

When these don't fully determine the output (→ `longest`, two inputs), you must annotate.

## Mental model

A reference is a "bookmark" into data someone else owns. The lifetime is the promise that the book
(the owned value) won't be thrown away while a bookmark still points into it. The borrow checker
enforces the promise at compile time.

## Recreate this

Write `first_word` (no `'a`), `longest<'a>` (explicit `'a`), and `Excerpt<'a>` (struct field
borrow). Then temporarily reproduce E0597 by borrowing a `String` from an inner scope and using it
after the scope closes — read all three spans of the error, then remove the broken code.
