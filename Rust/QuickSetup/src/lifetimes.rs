// Week 1 — Lifetimes deep-dive kata
// Goal: close the "explicit lifetimes" box.
//
// Three exercises:
//   1. first_word  — why elision lets us skip 'a        [DONE 2026-06-15]
//   2. longest     — why this one NEEDS 'a              [DONE 2026-06-15]
//   3. Excerpt<'a> — a struct holding a reference       [RESUME HERE next session]
//
// >>> NEXT SESSION: start at Exercise 3 — the TODO block at the bottom of this file.

pub(crate) fn lifetimes() {
    println!("--- lifetimes kata ---");
    let sentense = "hello world";
    let word = first_word(sentense);
    println!("first word of the sentense is {}", word);
    let longest_word = longest("hashan", "abc");
    println!("longest word is {}", longest_word);
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
// Exercise 3 — RESUME HERE (a struct that HOLDS a reference)
//
// You started this:
//     struct Excerpt<'a'> {   // <- typo: '<'a'>' parses as the CHAR literal 'a'.
//         part: &'a str       //    A lifetime is written <'a> — no closing quote.
//     }
// Correct declaration:  struct Excerpt<'a> { part: &'a str }
//
// WHY <'a> is mandatory here: the struct stores a *borrow*, not an owned String.
// 'a says "an Excerpt is only valid as long as the str it points into is alive,"
// so the compiler won't let an Excerpt outlive its source. There is NO elision for
// struct fields — if a struct holds a reference, you MUST write the lifetime.
// (C# bridge: this is basically a `ref struct` / Span<T> — can't outlive its source.)
//
// TASK:
//   1. Define:  struct Excerpt<'a> { part: &'a str }
//   2. In lifetimes(), add:
//        let novel = "Call me Ishmael. Some years ago.";
//        let first_sentence = novel.split('.').next().expect("no sentence found");
//        let excerpt = Excerpt { part: first_sentence };
//        println!("excerpt: {}", excerpt.part);   // -> "excerpt: Call me Ishmael"
//   3. Then break it on purpose to watch 'a fire (this should FAIL to compile):
//        let excerpt;
//        {
//            let novel = String::from("Call me Ishmael. Some years ago.");
//            excerpt = Excerpt { part: novel.split('.').next().unwrap() };
//        } // <- `novel` dropped here
//        println!("{}", excerpt.part); // dangling -> "`novel` does not live long enough"
//      Read that error (it's the whole point of lifetimes), then delete the bad block.
// ─────────────────────────────────────────────────────────────────────────────
