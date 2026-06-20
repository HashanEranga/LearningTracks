# 04 · Functions, Statements & Expressions

**Source:** `QuickSetup/src/functions_session.rs`

## Defining functions

`fn` with typed parameters and an optional `-> ReturnType`. Parameter types are always required.

```rust
fn functions_session(s: &str) {          // takes a string slice, returns nothing -> ()
    println!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("calculating multiplication of num1 {num1} and num2 {num2}");
    num1 * num2          // no semicolon → this is the return value
}
```

## The return value is the last expression

Rust functions return the value of their **final expression** — no `return` keyword needed (though
`return x;` exists for early returns). The catch is the semicolon:

- `num1 * num2` (no `;`) → an **expression** → its value is returned.
- `num1 * num2;` (with `;`) → a **statement** → evaluates to `()` → the function would return unit
  and likely fail to compile against a `-> i32` signature.

## Statements vs expressions (the core distinction)

- A **statement** performs an action and yields the unit value `()` — e.g. `let x = 5;`.
- An **expression** evaluates to a value — e.g. `5`, `num1 * num2`, `if a { 1 } else { 2 }`,
  a `match`, or a block `{ ... }`.

Adding a `;` to an expression turns it into a statement (discards its value). This is why the last
line of a function/block has no semicolon when you want to return it.

```rust
let full_name = {
    let first_name = "hashan";
    let last_name = "eranga";
    format!("{first_name} {last_name}")   // block's value (no ;) → bound to full_name
};
```

## Returning multiple values with a tuple

No "out params" — return a tuple and destructure at the call site.

```rust
fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 + num2, num2 - num1, num1 * num2)
}

let (add, sub, mul) = basic_math(12, 12);
```

## Format string interpolation

`println!`/`format!` support inline captures: `println!("{num1}")` reads the variable `num1`
directly. `{}` is Display, `{:?}` is Debug. Positional/empty `{}` still works:
`println!("{}", x)`.

## Recreate this

Write: a `fn` taking `&str` returning `()`; a `fn` returning `i32` via a trailing expression
(no semicolon); a `fn` returning a 3-tuple, then destructure it at the call site. Add a comment
explaining why removing the trailing `;` is what makes the value return.
