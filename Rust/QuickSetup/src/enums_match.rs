// Week 2 — Cluster 2: enums & exhaustive `match`
// Goal: model a value that is "one of a fixed set of shapes" and handle every shape safely.
//
// What this module demonstrates:
//   1. An enum whose variants carry DIFFERENT data (unit / tuple-style / struct-style).
//   2. `match` that the compiler forces to cover EVERY variant (exhaustiveness).
//   3. Pulling the data back OUT of a variant by binding it to names inside the pattern.
//   4. Match guards (`if`), the or-pattern (`|`), numeric ranges, and the `_` wildcard.
//   5. `if let` and `let ... else` as shorthands for a `match` that only cares about one case.
//
// Key takeaway: an enum makes the set of possibilities a *type*, and `match` makes the
// compiler prove you handled all of them. Forgetting a case is a compile error, not a bug
// you find in production.

// An enum lists every shape a `Command` is allowed to take. A value of type `Command` is
// ALWAYS exactly one of these variants at a time — never two, never none.
//
// Notice each variant carries a different payload:
enum Command {
    Get(String),                        // tuple-style: one unnamed field
    Set { key: String, value: String }, // struct-style: named fields
    Delete(String),                     // tuple-style again
    Ping,                               // unit-style: no data at all
}

// `match` on a `&Command` must produce an arm for every variant. If you delete any arm below,
// the program will not compile ("non-exhaustive patterns") — that compiler check is the whole
// point of using an enum + match.
//
// Each arm's pattern also BINDS the inner data to names (`key`, `value`) so we can use it.
fn apply(cmd: &Command) -> String {
    match cmd {
        // `key` names the String inside Get, borrowed from `cmd`.
        Command::Get(key) => format!("GET {}", key),
        // struct-style variants bind by field name.
        Command::Set { key, value } => format!("SET {} = {}", key, value),
        Command::Delete(key) => format!("DELETE {}", key),
        // a unit variant has no data, so there is nothing to bind.
        Command::Ping => String::from("PONG"),
    }
}

// A second `match`, this time over a plain integer, to show three more pattern tools:
//   - numeric ranges (`..=`)
//   - the or-pattern (`|`) to share one arm between several values
//   - the `_` wildcard for "everything else"
// Returning a borrowed `&'static str` is fine: these literals live for the whole program.
fn classify(code: u16) -> &'static str {
    match code {
        200 => "ok",
        // or-pattern: any one of these three matches this arm.
        301 | 302 | 307 => "redirect",
        // inclusive range: 400 through 499.
        400..=499 => "client error",
        500..=599 => "server error",
        // `_` is the catch-all. It is REQUIRED here because a u16 has thousands of values we
        // did not name — without it the match would not be exhaustive.
        _ => "unknown",
    }
}

// Match guards: an extra `if` condition on an arm. The arm only fires when its pattern matches
// AND the guard is true. Here we treat a Set with an empty value as a special case.
fn validate(cmd: &Command) -> &'static str {
    match cmd {
        // guard runs only after the `Set { .. }` pattern matched.
        Command::Set { value, .. } if value.is_empty() => "rejected: empty value",
        Command::Set { .. } => "accepted",
        // `_` collapses every remaining variant into one arm — fine when they share an outcome.
        _ => "not a write",
    }
}

pub(crate) fn enums_match() {
    println!("--- enums & match kata ---");

    let commands = [
        Command::Get(String::from("user:1")),
        Command::Set {
            key: String::from("user:1"),
            value: String::from("hashan"),
        },
        Command::Delete(String::from("user:1")),
        Command::Ping,
    ];

    // exhaustive match in action
    for cmd in &commands {
        println!("{}", apply(cmd));
    }

    // ranges + or-pattern + wildcard
    for code in [200u16, 302, 404, 503, 100] {
        println!("{} -> {}", code, classify(code));
    }

    // match guards
    let empty_set = Command::Set {
        key: String::from("k"),
        value: String::new(),
    };
    println!("{}", validate(&empty_set));
    println!("{}", validate(&commands[1])); // the good Set
    println!("{}", validate(&Command::Ping));

    // ── if let: a match that cares about ONLY ONE variant ────────────────────────
    // This is sugar for `match { Command::Get(k) => {..}, _ => {} }`. Use it when an
    // all-arms match would just be noise.
    let probe = Command::Get(String::from("session"));
    if let Command::Get(key) = &probe {
        println!("if let pulled out the key: {}", key);
    }

    // ── let ... else: bind, or bail out of the function on the "else" path ────────
    // If `probe` is a Get we keep `key`; otherwise we `return`. The else branch MUST
    // diverge (return / break / panic) — that is what lets `key` stay in scope after.
    let Command::Get(key) = &probe else {
        println!("not a Get, nothing to do");
        return;
    };
    println!("let-else kept the key in scope: {}", key);

    // ── self-check answers (pure Rust reasoning) ─────────────────────────────────
    // Q1: Why does removing the `Command::Ping` arm from `apply` fail to compile?
    //   A: `match` must be exhaustive — every possible variant needs an arm (or a `_`).
    //      A `Command` value could be `Ping`, so leaving it out means a case is unhandled,
    //      which is a compile error, not a runtime surprise.
    //
    // Q2: When is a `_` wildcard a smell?
    //   A: When matching an enum you control. `_` silently swallows variants you add LATER,
    //      so the compiler can no longer remind you to handle the new case. Prefer naming
    //      each variant for your own enums; reserve `_` for open sets like integers, or when
    //      several variants genuinely share one outcome.
}
