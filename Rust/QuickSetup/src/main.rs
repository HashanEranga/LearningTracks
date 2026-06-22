use crate::functions_session::basic_math;
mod borrowing;
mod compound_types;
mod conditions;
mod enums_match;
mod functions_session;
mod lifetimes;
mod loops;
mod option_result;
mod ownership;
mod traits_generics;

fn main() {
    println!("Hello, world!");
    compound_types::compound_types();

    functions_session::functions_session("hello world");
    let sample_string = "this is a sample string";
    functions_session::functions_session(sample_string);

    let answer = functions_session::multiplication(12, 12);
    println!("the answer is {}", answer);

    let (answer_add, answer_sub, answer_mul) = basic_math(12, 12);
    println!(
        "the answer_add {}, answer_sub {}, answer_mul {}",
        answer_add, answer_sub, answer_mul
    );

    // code blocks
    let full_name = {
        let first_name = "hashan";
        let last_name = "eranga";
        format!("{first_name} {last_name}")
    };

    println!("{full_name}");

    println!("conditions check");
    conditions::conditions();

    println!("loops check");
    loops::loops();

    // common for loop syntaxes
    // asc
    for i in 0..5 {
        println!("loop {}", i);
    }
    // desc
    for i in (0..5).rev() {
        println!("loop {}", i);
    }
    //stepwise
    for i in (0..=10).step_by(2) {
        println!("loop {}", i);
    }

    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];

    for (num, value) in pairs {
        println!("{} : {}", num, value);
    }

    // ownership
    println!("Ownership");
    ownership::ownership();

    // borrowing
    println!("Borrowing");
    borrowing::borrowing();

    // lifetimes
    println!("Lifetimes");
    lifetimes::lifetimes();

    // traits & generics (Week 2 — Cluster 1)
    println!("Traits & Generics");
    traits_generics::traits_generics();

    // enums & exhaustive match (Week 2 — Cluster 2)
    println!("Enums & Match");
    enums_match::enums_match();

    // Option / Result / ? (Week 2 — Cluster 3)
    println!("Option / Result / ?");
    option_result::option_result();
}
