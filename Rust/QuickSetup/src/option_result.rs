// Week 2 — Cluster 3: Option, Result & the `?` operator
// Goal: represent "maybe a value" and "succeeded or failed" without nulls or exceptions.
//
// What this module demonstrates:
//   1. `Option<T>` and `Result<T, E>` are just ENUMS from the standard library.
//   2. Handling them explicitly with `match`.
//   3. The everyday combinators: `map`, `unwrap_or`, `ok_or`, `and_then`, `.ok()`.
//   4. The `?` operator: unwrap-or-return-early, the backbone of error propagation.
//   5. `unwrap` / `expect`: extract the value but PANIC on the empty/error case.
//
// Key takeaway: absence and failure are ordinary values you must handle, checked by the
// compiler. There is no `null` to forget and no exception to silently bubble past you.

// `Option<T>` is defined in std as, essentially:
//     enum Option<T> { Some(T), None }
// `chars().next()` hands us one: `Some(first_char)` for a non-empty string, `None` otherwise.
fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

// `Result<T, E>` is defined in std as, essentially:
//     enum Result<T, E> { Ok(T), Err(E) }
// `str::parse` returns a Result because parsing can fail. The `?` after it means:
//   - if it's `Ok(n)`, unwrap `n` and keep going;
//   - if it's `Err(e)`, immediately `return Err(e)` from THIS function.
// `?` only works in a function whose return type can carry that error (here, the same
// `ParseIntError`).
fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    let port: u16 = s.trim().parse()?;
    Ok(port)
}

// `?` shines when several fallible steps chain: the happy path reads top-to-bottom, and any
// failure short-circuits out with its error. No nested `match` pyramid.
fn parse_pair(a: &str, b: &str) -> Result<(u16, u16), std::num::ParseIntError> {
    let x = parse_port(a)?;
    let y = parse_port(b)?;
    Ok((x, y))
}

// `ok_or` turns an `Option` into a `Result`: `Some(v)` becomes `Ok(v)`, and `None` becomes
// `Err(the value you pass)`. Handy for "absence here is actually an error."
fn first_char_or_err(s: &str) -> Result<char, &'static str> {
    s.chars().next().ok_or("empty string")
}

// `and_then` chains a second fallible step onto a `Some`: if the first is `None`, the whole
// thing is `None`; otherwise it runs your closure, which itself returns an `Option`.
// Here: get the first char, then try to read it as a base-10 digit (also fallible).
fn first_digit(s: &str) -> Option<u32> {
    s.chars().next().and_then(|c| c.to_digit(10))
}

pub(crate) fn option_result() {
    println!("--- option / result / ? kata ---");

    // ── match: handle both variants explicitly ───────────────────────────────────
    match first_char("hello") {
        Some(c) => println!("first char: {}", c),
        None => println!("empty string"),
    }
    match parse_port("8080") {
        Ok(p) => println!("parsed port: {}", p),
        Err(e) => println!("bad port: {}", e),
    }
    match parse_port("99999") {
        // 99999 > u16::MAX, so parse fails — the Err arm runs.
        Ok(p) => println!("parsed port: {}", p),
        Err(e) => println!("bad port: {}", e),
    }

    // ── combinators: handle the cases without writing match every time ────────────
    // map: transform the inner value, leave None/Err untouched.
    let upper = first_char("rust").map(|c| c.to_ascii_uppercase());
    println!("mapped: {:?}", upper); // Some('R')

    // unwrap_or: extract the value, or fall back to a default for None.
    let fallback = first_char("").unwrap_or('?');
    println!("unwrap_or: {}", fallback); // ?

    // .ok(): throw away a Result's error, keeping just Some/None.
    println!("as option: {:?}", parse_port("70").ok()); // Some(70)
    println!("as option: {:?}", parse_port("oops").ok()); // None

    // ok_or: Option -> Result
    println!("ok_or ok:  {:?}", first_char_or_err("hi"));
    println!("ok_or err: {:?}", first_char_or_err(""));

    // and_then: chain a second fallible step
    println!("first_digit('7x'): {:?}", first_digit("7x")); // Some(7)
    println!("first_digit('ax'): {:?}", first_digit("ax")); // None (not a digit)

    // ── ? propagation across several steps ────────────────────────────────────────
    println!("parse_pair ok:  {:?}", parse_pair("80", "443"));
    println!("parse_pair err: {:?}", parse_pair("80", "nope"));

    // ── unwrap / expect: extract or PANIC ─────────────────────────────────────────
    // Use these ONLY when the empty/error case would mean a bug in your program — never
    // for input you expect to be wrong. `expect` lets you record the invariant you relied
    // on, so the panic message explains *why* you thought it couldn't fail.
    let definitely = parse_port("443").expect("443 is a valid u16 port");
    println!("expect (safe here): {}", definitely);

    // ── self-check answers (pure Rust reasoning) ─────────────────────────────────
    // Q1: What does `?` actually expand to?
    //   A: `let x = expr?;` is roughly
    //        `let x = match expr { Ok(v) => v, Err(e) => return Err(e.into()) };`
    //      (for Option: `Some(v) => v, None => return None`). It unwraps the success value
    //      or returns the failure from the current function — which is why the function's
    //      return type has to be able to hold that failure.
    //
    // Q2: When is `unwrap`/`expect` acceptable vs a code smell?
    //   A: Acceptable when None/Err is genuinely impossible given an invariant you control
    //      (prefer `expect` to document that invariant). A smell when used on real fallible
    //      input — that turns a recoverable error into a crash; handle it with match / `?` /
    //      a combinator instead.
}
