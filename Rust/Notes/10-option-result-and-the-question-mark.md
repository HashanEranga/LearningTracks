# 10 · Option, Result & the `?` Operator

**Source:** `QuickSetup/src/option_result.rs`

> Builds directly on [Note 09 — Enums & pattern matching](./09-enums-and-pattern-matching.md):
> `Option` and `Result` are just enums, and everything you learned about `match` applies to them.

## The problem this solves

Two situations come up constantly:

1. **A value might not be there.** The first character of a string — but the string could be empty.
2. **An operation might fail.** Parsing `"8080"` into a number works; parsing `"oops"` doesn't.

Many languages model these with a special "nothing" value (`null`) and with exceptions that unwind
the stack. Both let a problem travel silently: a `null` flows along until something dereferences it
and crashes far from the cause; an exception sails past every function that didn't think to catch
it. Rust has neither. Instead it makes "maybe missing" and "maybe failed" into **ordinary values of
ordinary enum types** — and the compiler won't let you use the inside until you've said what happens
in the empty/failed case.

## Step 1 — The two enums

They are nothing magic — just enums from the standard library:

```rust
enum Option<T> { Some(T), None }          // a value, or nothing
enum Result<T, E> { Ok(T), Err(E) }       // a success value, or an error value
```

`Option<char>` is "maybe a `char`." `Result<u16, ParseIntError>` is "a `u16` on success, or a
`ParseIntError` describing why it failed." Because they're enums, you already know how to take them
apart: `match`.

## Step 2 — Handle both variants with `match`

```rust
fn first_char(s: &str) -> Option<char> {
    s.chars().next()          // Some(c) for non-empty, None for ""
}

match first_char("hello") {
    Some(c) => println!("first char: {}", c),
    None    => println!("empty string"),
}
```

```rust
fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    let port: u16 = s.trim().parse()?;   // (the `?` is Step 4 — read on)
    Ok(port)
}

match parse_port("8080") {
    Ok(p)  => println!("parsed port: {}", p),
    Err(e) => println!("bad port: {}", e),
}
```

`match` is exhaustive (Note 09), so the compiler guarantees you handled *both* `Some`/`None` (or
`Ok`/`Err`). You cannot accidentally use a missing value.

## Step 3 — Combinators (skip the `match` when it's just ceremony)

Writing a full `match` every time gets noisy. These methods do the common cases in one line. Each
one leaves the empty/error variant untouched and only acts on the value:

| Method | On | Does |
|---|---|---|
| `map(f)` | `Option`/`Result` | transform the inner value; `None`/`Err` pass through unchanged |
| `unwrap_or(default)` | `Option`/`Result` | the value, or a fallback if empty/error |
| `.ok()` | `Result` | drop the error, becoming `Some`/`None` |
| `ok_or(err)` | `Option` | `Some(v)`→`Ok(v)`, `None`→`Err(err)` — turn absence into an error |
| `and_then(f)` | `Option`/`Result` | chain a *second* fallible step; short-circuits on empty/error |

```rust
first_char("rust").map(|c| c.to_ascii_uppercase());  // Some('R')
first_char("").unwrap_or('?');                        // '?'
parse_port("oops").ok();                              // None
first_char_or_err("");                                // Err("empty string")  (via ok_or)
"7x".chars().next().and_then(|c| c.to_digit(10));     // Some(7)
"ax".chars().next().and_then(|c| c.to_digit(10));     // None  (first char isn't a digit)
```

`map` vs `and_then`: use **`map`** when your closure returns a plain value; use **`and_then`** when
your closure *itself* returns an `Option`/`Result` (otherwise you'd end up with a nested
`Option<Option<_>>`).

## Step 4 — The `?` operator (unwrap, or return the failure)

The single most-used tool. Look again at `parse_port`:

```rust
let port: u16 = s.trim().parse()?;
```

`parse()` returns a `Result`. The `?` says:

- if it's `Ok(n)` → unwrap `n` and carry on;
- if it's `Err(e)` → **return `Err(e)` from the whole function, right here.**

So `?` is shorthand for this `match`:

```rust
let port: u16 = match s.trim().parse() {
    Ok(n)  => n,
    Err(e) => return Err(e),   // for Option: None => return None
};
```

Because `?` can *return* the error, it only compiles inside a function whose return type can hold
it (here both are `Result<_, ParseIntError>`). The payoff is that a chain of fallible steps reads
like ordinary straight-line code, and any failure exits early with its error:

```rust
fn parse_pair(a: &str, b: &str) -> Result<(u16, u16), std::num::ParseIntError> {
    let x = parse_port(a)?;   // bail out here if `a` is bad
    let y = parse_port(b)?;   // …or here if `b` is bad
    Ok((x, y))
}
```

No pyramid of nested matches — the happy path is the visible path.

## Step 5 — `unwrap` / `expect` (extract, or crash)

```rust
let definitely = parse_port("443").expect("443 is a valid u16 port");
```

`unwrap()` and `expect(msg)` pull the value out of `Ok`/`Some` — but if it's `Err`/`None` they
**panic** (crash the thread). Rules of thumb:

- **Fine** when the empty/error case is genuinely impossible because of an invariant *you* control
  — and prefer `expect` over `unwrap` so the panic message records *why* you believed it couldn't
  fail.
- **A smell** on real fallible input (user data, files, network). That converts a recoverable error
  into a crash. Handle it with `match`, a combinator, or `?` instead.

## Expected output

Running `option_result()` prints:

```text
--- option / result / ? kata ---
first char: h
parsed port: 8080
bad port: number too large to fit in target type
mapped: Some('R')
unwrap_or: ?
as option: Some(70)
as option: None
ok_or ok:  Ok('h')
ok_or err: Err("empty string")
first_digit('7x'): Some(7)
first_digit('ax'): None
parse_pair ok:  Ok((80, 443))
parse_pair err: Err(ParseIntError { kind: InvalidDigit })
expect (safe here): 443
```

## The rules to remember

- `Option<T>` = maybe a value (`Some`/`None`); `Result<T, E>` = success or error (`Ok`/`Err`). Both
  are enums — handle them with `match`.
- Reach for combinators (`map`, `unwrap_or`, `ok_or`, `and_then`, `.ok()`) for the common one-liners.
- `?` = "unwrap the success, or return the failure from this function." It's the backbone of error
  handling; the function's return type must be able to carry the error.
- `unwrap`/`expect` panic on the bad case — only use them when that case is a bug, and prefer
  `expect` with a reason.

## Recreate this

1. Write `first_char(s) -> Option<char>` with `s.chars().next()`, and handle it with a `match`.
2. Write `parse_port(s) -> Result<u16, ParseIntError>` using `?`, and handle it with a `match`
   (try a too-large input so the `Err` arm runs).
3. Demonstrate `map`, `unwrap_or`, and `.ok()` on those, printing each result with `{:?}`.
4. Write `first_char_or_err` using `ok_or`, and `first_digit` using `and_then(|c| c.to_digit(10))`.
5. Write `parse_pair(a, b)` that calls `parse_port` twice with `?` and returns a tuple.
6. Call `parse_port("443").expect(...)` and, in a comment, explain what `?` expands to and when
   `unwrap`/`expect` are acceptable.
