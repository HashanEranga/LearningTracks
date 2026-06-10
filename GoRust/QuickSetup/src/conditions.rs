pub(crate) fn conditions(){
    let grade = 56;

    println!("if else conditions");
    if grade >= 50 {
        println!("passed");
    } else {
        println!("failed");
    }

    println!("if else ladder");
    if grade >= 75 {
        println!("A");
    } else if grade >= 65 {
        println!("B");
    } else if grade >= 50 {
        println!("C");
    } else {
        println!("D");
    }
    
    println!("match conditioning");
    match grade {
        75..=100 => println!("A"),
        65..=74 => println!("B"),
        50..=64 => println!("C"),
        35..=49 => println!("D"),
        _ => println!("E"),
    }
}