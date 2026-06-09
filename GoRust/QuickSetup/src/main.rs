use crate::functions_session::basic_math;

mod compound_types;
mod functions_session;

fn main() {
    println!("Hello, world!");
    compound_types::compound_types();
    
    functions_session::functions_session("hello world");
    let sample_string = "this is a sample string";
    functions_session::functions_session(sample_string);
    
    let answer = functions_session::multiplication(12,12);
    println!("the answer is {}", answer);
    
    let (answer_add, answer_sub, answer_mul) = basic_math(12, 12);
    println!("the answer_add {}, answer_sub {}, answer_mul {}", answer_add, answer_sub, answer_mul);

    // code blocks
    let full_name = {
        let first_name = "hashan";
        let last_name = "eranga";
        format!("{first_name} {last_name}")
    };

    println!("{full_name}");
}
