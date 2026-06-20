// Week 2 — Cluster 1: traits, bounds, and dyn vs generics
// Goal: close the "dyn vs generics" half of the Week-2 self-check with a runnable artifact.
//
// What this module demonstrates:
//   1. A trait with a required method + a default method.
//   2. Two unrelated structs implementing the same trait.
//   3. announce_static<T: Describe> — STATIC dispatch (generics, monomorphized).
//   4. announce_dyn(&[&dyn Describe]) — DYNAMIC dispatch (trait objects, vtable).
//
// Key takeaway: a generic `<T: Describe>` is specialized by the compiler into one copy
// per concrete type (zero-cost, inlinable). A `&dyn Describe` erases the type and calls
// through a vtable at runtime — slower, but the only way to mix different types behind the
// same trait inside one collection.

// A trait = a named set of behaviour a type can provide.
trait Describe {
    // Required: every implementor MUST provide this.
    fn describe(&self) -> String;

    // Default: implementors get this for free unless they override it.
    // It is written in terms of the required method.
    fn shout(&self) -> String {
        format!("{}!!!", self.describe())
    }
}

struct Server {
    name: String,
    port: u16,
}

struct User {
    handle: String,
}

impl Describe for Server {
    fn describe(&self) -> String {
        format!("server {} on port {}", self.name, self.port)
    }
}

impl Describe for User {
    fn describe(&self) -> String {
        // No shout() here — Server and User both inherit the default.
        format!("user @{}", self.handle)
    }
}

// STATIC dispatch: the bound `T: Describe` says "any type that implements Describe".
// The compiler stamps out a dedicated copy of this function for each concrete T it is
// called with (monomorphization). The call to describe() is resolved at compile time and
// can be inlined — there is no runtime type information involved.
fn announce_static<T: Describe>(item: &T) {
    println!("[static] {}", item.describe());
}

// DYNAMIC dispatch: `&dyn Describe` is a trait object — a fat pointer carrying (a) the data
// pointer and (b) a pointer to a vtable of that type's Describe methods. The concrete type
// is erased, so a single slice can hold a mix of Server and User. The describe() call is
// looked up in the vtable at runtime.
fn announce_dyn(items: &[&dyn Describe]) {
    for item in items {
        println!("[dyn]    {} / {}", item.describe(), item.shout());
    }
}

pub(crate) fn traits_generics() {
    println!("--- traits & generics kata ---");

    let api = Server {
        name: String::from("api"),
        port: 8080,
    };
    let hashan = User {
        handle: String::from("hashan"),
    };

    // Static dispatch: each call below is a different monomorphized instantiation
    // (one for Server, one for User).
    announce_static(&api);
    announce_static(&hashan);

    // Dynamic dispatch: a SINGLE slice holding two different concrete types, unified
    // behind `&dyn Describe`. This is the thing generics alone cannot express.
    let mixed: [&dyn Describe; 2] = [&api, &hashan];
    announce_dyn(&mixed);

    // ── self-check answers (pure Rust reasoning) ─────────────────────────────────
    // Q1: Why does the mixed slice compile with `&dyn Describe` but a `&[T]` generic
    //     version would not?
    //   A: A generic `&[T]` is monomorphized to ONE concrete T per call, so every
    //      element must be the exact same type. `&dyn Describe` erases the concrete
    //      type behind a uniform fat pointer, so Server and User become the same type
    //      (`&dyn Describe`) at the slice level and can coexist.
    //
    // Q2: Which form allocates / uses a vtable?
    //   A: The `dyn` form. Each `&dyn Describe` carries a pointer to a per-type vtable
    //      and dispatches through it at runtime. The generic form has no vtable — calls
    //      are resolved statically at compile time and can inline.
}
