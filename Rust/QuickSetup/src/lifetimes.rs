// Week 1 — Lifetimes deep-dive kata
// Goal: close the "explicit lifetimes" box.
//
// Three exercises:
//   1. first_word  — why elision lets us skip 'a        [DONE 2026-06-15]
//   2. longest     — why this one NEEDS 'a              [DONE 2026-06-15]
//   3. Excerpt<'a> — a struct holding a reference       [DONE 2026-06-18]
//
// All three done. Takeaway: a struct field that is a reference forces an explicit <'a>
// (no elision for structs); E0597 fires at the borrow site when the borrowed source is
// dropped before the borrow's last use.

struct Excerpt<'a> {
    part: &'a str
}

pub(crate) fn lifetimes() {
    println!("--- lifetimes kata ---");
    let sentense = "hello world";
    let word = first_word(sentense);
    println!("first word of the sentense is {}", word);
    let longest_word = longest("hashan", "abc");
    println!("longest word is {}", longest_word);

    let novel = "Call me Ishamael. Some years ago.";
    let first_sentence = novel.split('.').next().expect("no sentence found");
    let excerpt = Excerpt { part : first_sentence};

    println!("excerpt : {}", excerpt.part);
    // Task 3 (dangling Excerpt) verified to FAIL with E0597 on 2026-06-18, then removed.
    // The borrow checker blamed the borrow-creation line, with `novel` dropped at the
    // scope close + the later `println!` as the reason the borrow had to outlive it.

    // ── move vs borrow vs clone (Week-1 self-check) ───────────────────────────
    // Default for a non-Copy type is MOVE: `let t = s;` transfers ownership and `s`
    // becomes unusable — Rust's way of guaranteeing memory safety without a GC.
    // When you still need the value, the choice depends on intent, and the three
    // options live at two different levels:
    //   • borrow / clone are USE-SITE choices, made per call:
    //       borrow = "you keep it, I'll just look" (zero alloc — the default reach)
    //       clone  = "I need my own independent heap copy" (costs an allocation)
    //   • Copy is a TYPE-LEVEL property, set once via derive: for trivial stack types
    //       a move leaves the original intact, so assignment just copies it.
    let s = String::from("Hashan");
    // let t = s;            // MOVE — would make `s` unusable below (ownership error)
    // println!("{s}");
    
    // fix 1 - clone the value - make an explicit copy (new heap allocation)
    println!("Fix 1 - clone the value");
    let cloned_s = s.clone();
    
    println!("{} {}", s, cloned_s);
    
    // fix 2 use a reference - borrow instead of moving (zero alloc; `s` stays usable)
    println!("Fix 2 - use a reference");
    let t = &s;
    println!("{} - {}", s, t);
    
    // fix 3 Derive Copy - trivial stack types (no heap/Drop): assignment copies, original stays valid
    println!("Fix 3 - Derive a copy");
    #[derive(Copy, Clone)]
    struct Point { x: i32, y: i32 }

    let p1 = Point { x: 1, y: 2};
    let p2 = p1; 

    println!("Point 1 X value : {}, Y value : {}", p1.x, p1.y);
    println!("Point 2 X value : {}, Y value : {}", p2.x, p2.y);
}

// Ex 1 — one input reference, so elision ties the output's lifetime to it (no 'a).
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

// Ex 2 — two input refs, output could borrow from either -> compiler can't infer,
// so we declare a shared lifetime 'a (result valid for the *shorter* of a and b).
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ex 3 — Excerpt<'a>: a struct that HOLDS a reference (see definition at top).
//
// WHY <'a> is mandatory: the struct stores a *borrow*, not an owned String. 'a says
// "an Excerpt is only valid as long as the str it points into is alive," so the
// compiler won't let an Excerpt outlive its source. There is NO elision for struct
// fields — if a struct holds a reference, you MUST write the lifetime.
// (C# bridge: like a `ref struct` / Span<T> — can't outlive its source. No GC to
//  keep the backing data alive; the String is dropped deterministically at scope end.)
//
// Verified the failure (E0597 "`novel` does not live long enough"): borrow a `String`
// created in an inner scope, then use the Excerpt after the scope closes. rustc blames
// the borrow-creation line (^^^^^), with "dropped here while still borrowed" at the
// closing brace and "borrow later used here" at the final println — the reason the
// borrow had to outlive its source. A "literal" source is &'static str, so it never fires.
// ─────────────────────────────────────────────────────────────────────────────
