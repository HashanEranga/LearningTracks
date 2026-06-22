# 09 · Enums & Pattern Matching

**Source:** `QuickSetup/src/enums_match.rs`

## The problem this solves

A lot of real values are *"one of a fixed set of shapes."* A command sent to a key-value store is
either a read, a write, a delete, or a health-check ping — never two at once, never something
outside that list. If you model that with loose pieces — a string for the "kind", plus some
optional fields that are only meaningful for *some* kinds — you end up writing defensive checks
everywhere ("is this a Set? then `value` should be filled in… I hope"), and nothing stops you from
building a nonsensical value.

An **enum** lets you say, up front and as a *type*: "a `Command` is exactly one of these four
shapes, and each shape carries exactly the data it needs." Then `match` makes the compiler force
you to handle every shape. Forgetting one becomes a compile error instead of a production bug.

## Step 1 — Define the set of shapes (the enum)

```rust
enum Command {
    Get(String),                        // tuple-style: one unnamed field
    Set { key: String, value: String }, // struct-style: named fields
    Delete(String),                     // tuple-style again
    Ping,                               // unit-style: no data at all
}
```

A value of type `Command` is **always exactly one** of these four **variants**. The important part:
each variant can carry a *different* payload, and there are three flavors of payload:

- **Unit-style** (`Ping`) — no data. The variant *is* the whole story.
- **Tuple-style** (`Get(String)`, `Delete(String)`) — one or more unnamed fields, positional.
- **Struct-style** (`Set { key, value }`) — named fields, like a little struct attached to the
  variant.

You can mix all three in one enum, as above. This is the thing a plain "kind string + loose fields"
design can't do: the data each case needs is *welded to that case*.

## Step 2 — Handle every shape with `match`

```rust
fn apply(cmd: &Command) -> String {
    match cmd {
        Command::Get(key) => format!("GET {}", key),
        Command::Set { key, value } => format!("SET {} = {}", key, value),
        Command::Delete(key) => format!("DELETE {}", key),
        Command::Ping => String::from("PONG"),
    }
}
```

Two things are happening in each arm:

1. **The pattern picks the variant.** `Command::Get(...)` only matches a `Get`.
2. **The pattern binds the data.** Inside `Command::Get(key)`, the name `key` is bound to the
   `String` that this `Get` is carrying, so you can use it on the right of `=>`. For the
   struct-style variant you bind by field name: `Command::Set { key, value }`. A unit variant
   (`Ping`) has nothing to bind.

**Exhaustiveness — the payoff.** A `match` must cover *every* variant. Delete the `Command::Ping`
arm and the code will not compile:

```text
error[E0004]: non-exhaustive patterns: `Command::Ping` not covered
```

That's the compiler proving, at build time, that you handled all four cases. When you later add a
fifth variant, every `match` that forgot it lights up red — the compiler hands you a to-do list.

## Step 3 — Three more pattern tools (ranges, or-patterns, wildcard)

`match` works on more than enums. Matching an integer shows three tools you'll reuse everywhere:

```rust
fn classify(code: u16) -> &'static str {
    match code {
        200 => "ok",
        301 | 302 | 307 => "redirect",   // or-pattern: any of these
        400..=499 => "client error",     // inclusive range 400 through 499
        500..=599 => "server error",
        _ => "unknown",                   // wildcard: everything else
    }
}
```

- **Or-pattern `|`** — one arm shared by several values. `301 | 302 | 307` reads "if the code is
  301 *or* 302 *or* 307."
- **Inclusive range `..=`** — `400..=499` matches every value from 400 to 499 *including* 499.
- **Wildcard `_`** — matches anything not already matched. It's **required** here: a `u16` has tens
  of thousands of possible values and we only named a few, so without `_` the match isn't
  exhaustive.

(`&'static str` as the return type just means "a string that lives for the whole program" — string
literals like `"ok"` do, so returning them is free.)

## Step 4 — Match guards (an extra condition on an arm)

A **guard** is an `if` tacked onto an arm. The arm fires only when the pattern matches *and* the
guard is true:

```rust
fn validate(cmd: &Command) -> &'static str {
    match cmd {
        Command::Set { value, .. } if value.is_empty() => "rejected: empty value",
        Command::Set { .. } => "accepted",
        _ => "not a write",
    }
}
```

Read the first arm as: "a `Set` *whose `value` is empty*." The `..` means "and I don't care about
the other fields." Order matters — the guarded arm is tried first, so a `Set` with an empty value
is rejected; any other `Set` falls through to "accepted." Everything else hits the `_`.

## Step 5 — `if let` and `let … else` (when you only care about one case)

Sometimes a full `match` is overkill because you only care about *one* variant. Two shorthands:

**`if let`** — "if it matches this one pattern, run this block":

```rust
let probe = Command::Get(String::from("session"));
if let Command::Get(key) = &probe {
    println!("if let pulled out the key: {}", key);
}
```

This is exactly `match &probe { Command::Get(key) => { … }, _ => {} }`, just without the noise.

**`let … else`** — "bind it, or bail out":

```rust
let Command::Get(key) = &probe else {
    println!("not a Get, nothing to do");
    return;
};
println!("let-else kept the key in scope: {}", key);
```

If `probe` is a `Get`, `key` is bound and stays usable for the rest of the function. If it isn't,
the `else` block runs — and that block **must diverge** (`return`, `break`, `panic!`, …). That
divergence is what lets the compiler guarantee `key` exists on every line after the `let`.

## Expected output

Running `enums_match()` prints:

```text
--- enums & match kata ---
GET user:1
SET user:1 = hashan
DELETE user:1
PONG
200 -> ok
302 -> redirect
404 -> client error
503 -> server error
100 -> unknown
rejected: empty value
accepted
not a write
if let pulled out the key: session
let-else kept the key in scope: session
```

## The rules to remember

| Want to… | Use |
|---|---|
| Model "one of a fixed set of shapes" | an **enum** with data-carrying variants |
| Handle every shape, checked at compile time | **`match`** (exhaustive) |
| Share one arm across several values | the **or-pattern** `a | b | c` |
| Match a span of values | a **range** `lo..=hi` |
| Add a condition to an arm | a **guard** `pattern if cond` |
| React to just one variant | **`if let`** |
| Bind one variant or bail | **`let … else`** |

Prefer naming each variant over a blanket `_` for enums *you* own — that way the compiler reminds
you when a newly added variant needs handling.

## Recreate this

1. Define `enum Command` with `Get(String)`, `Set { key, value }`, `Delete(String)`, and `Ping`
   (one of each payload flavor).
2. Write `apply(cmd: &Command) -> String` with an exhaustive `match`, binding the inner data in
   each arm. Confirm that deleting the `Ping` arm fails to compile.
3. Write `classify(code: u16) -> &'static str` using an or-pattern, two inclusive ranges, and a
   `_` wildcard.
4. Write `validate(cmd: &Command) -> &'static str` with a **guard** that rejects a `Set` whose
   `value.is_empty()`.
5. Use `if let` to react to a single `Get`, and `let … else` to bind a `Get`'s key or `return`.
6. In a comment, answer: *why is a blanket `_` a smell when matching an enum you own?*
