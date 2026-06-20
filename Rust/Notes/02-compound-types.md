# 02 · Compound Types

**Source:** `QuickSetup/src/compound_types.rs`

Covers the everyday container types: string types, arrays, vectors, tuples, and the unit type.

## Strings: `&str` vs `String`

Two different types, and the distinction matters constantly in Rust.

```rust
let fixed_str = "fixed length string";              // &str (specifically &'static str)
let mut variable_string = String::from("variable length string"); // String
variable_string.push('s');                          // grows — needs `mut`
```

- **`&str`** — a *string slice*: an immutable, borrowed **view** into UTF-8 text. A string literal
  like `"hello"` is baked into the binary, so its type is `&'static str`. You can't grow it.
- **`String`** — an **owned**, heap-allocated, growable buffer. You own it, you can mutate it
  (`push`, `push_str`), and it's freed when it goes out of scope.

Rule of thumb: take `&str` as a function parameter (accepts both literals and `String` via deref),
return/store `String` when you need ownership.

## Arrays `[T; N]`

Fixed-size, all elements the same type, length is part of the type, lives on the stack.

```rust
let num_array = [10, 20, 30, 40, 50];   // [i32; 5]
let zero_array = [0; 10];               // [i32; 10], all zeros — [value; count]
```

The length `N` is compile-time fixed. You cannot push/grow an array — for that you need a `Vec`.

## Vectors `Vec<T>`

Growable, heap-allocated, same element type. The workhorse collection.

```rust
let vector = vec![1, 2, 3, 4, 5];   // vec! macro builds a Vec<i32>
```

(See also `Vec::new()` + `push` in `ownership.rs`.)

## Tuples

Fixed-length, **heterogeneous** — each position can be a different type.

```rust
let sample_tuple = ("salary", 400, "age", 28);     // (&str, i32, &str, i32)
let (salary, salary_value, age, age_value) = sample_tuple;   // destructuring
println!("{}", sample_tuple.1);                    // index access with .N
```

## The unit type `()`

The empty tuple. **Zero size**, holds no data, occupies no memory. It's what expressions and
functions "return" when they have nothing meaningful to give back (e.g. a function with no
explicit return value returns `()`).

```rust
let unit = ();   // type is ()
```

## Debug printing

`{:?}` is the **Debug** format (compact); `{:#?}` is the pretty/multi-line Debug. Arrays, vectors
and tuples all implement Debug, so use `{:?}` to print them — plain `{}` (Display) won't work for
these containers.

## Recreate this

In `compound_types()`: build a `&str` and a growable `String` (and `push` to it), a fixed array
and a `[0; N]` array, a `vec!`, a tuple (then destructure it), and a unit `()`. Print each with
`{:?}` / `{:#?}` where needed.
