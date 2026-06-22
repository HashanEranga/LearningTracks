pub(crate) fn functions_session(s: &str) {
    println!("{s}");
}

pub(crate) fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("calculating multiplication of num1 {num1} and num2 {num2}");
    num1 * num2
}

pub(crate) fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 + num2, num2 - num1, num1 * num2)
}

// expression vs statement
// statement always returns a unit value
// expression returns a valid value including a unit value
