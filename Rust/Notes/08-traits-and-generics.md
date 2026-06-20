# 08 · Traits & Generics

**Source:** `QuickSetup/src/traits_generics.rs`

## The problem this solves

Imagine you have two completely different types — a `Server` and a `User`. They have nothing in
common structurally:

```rust
struct Server { name: String, port: u16 }
struct User   { handle: String }
```

But you want to do *the same thing* to both: print a short description of each. Without traits,
you'd write separate functions (`describe_server`, `describe_user`) and separate "print" functions,
duplicating logic for every type. Traits let you say "these unrelated types share **one common
ability**", and then write code against that ability instead of against each type.

## Step 1 — Define the shared ability (the trait)

A trait is a named promise: "any type that implements me can do these things." Think of it as a
checklist of methods.

```rust
trait Describe {
    fn describe(&self) -> String;          // (a) required — no body here
    fn shout(&self) -> String {            // (b) default — has a body
        format!("{}!!!", self.describe())
    }
}
```

Two kinds of methods, and the difference matters:

- **(a) `describe` is *required*.** There's no `{ ... }` body — just the signature ending in `;`.
  This is the trait saying: *"I don't know how each type describes itself — every type has to tell
  me."* So each type is forced to write its own `describe`.
- **(b) `shout` is a *default*.** It has a real body. The trait already knows how to shout *if* it
  can describe: take the description and add `!!!`. Any type that implements `Describe` gets `shout`
  for free, without writing it. (A type *could* override it, but ours don't.)

Notice `shout` calls `self.describe()` even though the trait doesn't yet know what `describe`
returns for any specific type. That's fine — at the moment a real type uses it, `self.describe()`
resolves to *that type's* version.

## Step 2 — Let each type fulfill the promise (`impl`)

```rust
impl Describe for Server {
    fn describe(&self) -> String {
        format!("server {} on port {}", self.name, self.port)
    }
}

impl Describe for User {
    fn describe(&self) -> String {
        format!("user @{}", self.handle)
    }
}
```

`impl Describe for Server` reads: *"here is how `Server` satisfies the `Describe` checklist."* Each
type only writes the **required** method (`describe`); both automatically receive `shout`.

Now both types can be used like this:

```rust
let api = Server { name: String::from("api"), port: 8080 };
let me  = User   { handle: String::from("hashan") };

println!("{}", api.describe());   // server api on port 8080
println!("{}", me.shout());       // user @hashan!!!   <- the free default method
```

`api.shout()` would print `server api on port 8080!!!` — the default `shout` plugged in `Server`'s
own `describe`.

## Step 3 — Write ONE function that works for any `Describe` type

This is the real goal: a single function you can hand *either* a `Server` or a `User`. There are two
ways to write it. Understanding the difference between them is the whole point of this topic.

### Way 1 — Generics: `<T: Describe>`

```rust
fn announce_static<T: Describe>(item: &T) {
    println!("{}", item.describe());
}
```

How to read `<T: Describe>`: *"`T` stands for some type — I don't care which — **as long as** it
implements `Describe`."* The `: Describe` part is a **requirement** (called a *trait bound*). It's
what lets you call `item.describe()` inside: the compiler knows any `T` allowed here definitely has
`describe`.

**What actually happens when you call it** — this is the part that makes generics click:

```rust
announce_static(&api);   // api is a Server
announce_static(&me);    // me  is a User
```

The compiler sees you called it with a `Server` and with a `User`, so behind your back it generates
**two real, separate functions** — roughly as if you had written:

```rust
fn announce_static_for_Server(item: &Server) { println!("{}", item.describe()); }
fn announce_static_for_User(item: &User)     { println!("{}", item.describe()); }
```

You wrote one generic function; the compiler stamped out one concrete copy per type you used. Each
copy calls the exact `describe` for its type directly — there's no "figuring out which type this is"
at runtime, because each copy already knows. That makes it as fast as hand-writing both. This is the
**normal, default** way to be generic in Rust.

### Way 2 — Trait objects: `&dyn Describe`

```rust
fn announce_dyn(items: &[&dyn Describe]) {
    for item in items {
        println!("{} / {}", item.describe(), item.shout());
    }
}
```

`&dyn Describe` means: *"a reference to **some** value that implements `Describe`, but I won't
commit to which concrete type."* The word `dyn` is a flag that says "decide the actual method to
call later, while running" — this is called **dynamic dispatch**, and such a value is a **trait
object**.

Here there is only **one** copy of `announce_dyn` (the compiler does *not* stamp out a per-type
version). When the loop hits `item.describe()`, the program looks at the actual value it's holding
*at that moment* and runs the matching `describe` — `Server`'s or `User`'s. That lookup costs a
tiny bit more than the generic version, but it buys something generics cannot give you:

```rust
let mixed: [&dyn Describe; 2] = [&api, &me];   // a Server AND a User in ONE array
announce_dyn(&mixed);
```

Output:

```text
server api on port 8080 / server api on port 8080!!!
user @hashan / user @hashan!!!
```

## Step 4 — Why the mixed array *needs* `dyn`

Go back to how generics work: the compiler picks **one** concrete type per use. So a generic array
`[T; 2]` would have to be `[Server; 2]` *or* `[User; 2]` — every slot the same type. You cannot put
a `Server` and a `User` in the same generic array, because there's no single `T` that is both.

`&dyn Describe` erases the exact type. From the array's point of view, every element is just "a
`Describe`-thing" — so a `Server` and a `User` are now the *same* type (`&dyn Describe`) and can sit
side by side. That's the trade: you give up knowing the exact type (and a little speed) to gain the
freedom to mix types.

## The rule to remember

| Question | Use |
|---|---|
| Just need it to work for any one type per call? | **Generics** `<T: Describe>` (default, faster) |
| Need to mix different types in one list/array? | **`dyn`** `&dyn Describe` |

Default to generics. Reach for `dyn` only when you specifically need a single collection holding
different concrete types.

## Recreate this

1. Define `trait Describe` with a **required** `describe(&self) -> String` and a **default**
   `shout(&self) -> String` that reuses `describe`.
2. Implement it for `Server { name, port }` and `User { handle }` — write only `describe` for each.
3. Write `announce_static<T: Describe>(item: &T)` and call it with a `Server` and a `User`.
4. Write `announce_dyn(items: &[&dyn Describe])` and pass it an array holding **both** a `Server`
   and a `User`.
5. In a comment, answer: *why can the `dyn` array hold both types when a generic array can't?*
