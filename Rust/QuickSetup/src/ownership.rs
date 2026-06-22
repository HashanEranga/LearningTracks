pub(crate) fn ownership() {
    // ownership basics
    // 1 Each value has a variable that's its "owner"
    // 2 A value can only have one owner at a time
    // 3 if the owner goes out of scope, value is cleaned up

    let s1 = String::from("world");
    let s2 = s1;

    // println!("s1 is {s1}");
    println!("s2 is {s2}");

    // functions that take ownership
    let sample_vec = vec![1, 2, 3, 4, 5];
    take_ownership(sample_vec);
    // println!("vector elements in main {:?}", sample_vec);

    let sample_vec_two = gives_ownership();
    println!("sample vec two elements are {:?}", sample_vec_two);

    let sample_vec_three = takes_and_gives_ownership(sample_vec_two);
    println!("sample vec three elements are {:?}", sample_vec_three);
}

fn take_ownership(vec: Vec<i32>) {
    println!("vector elements in the method {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    let mut sample_vec = Vec::new();
    sample_vec.push(1);
    sample_vec.push(2);
    sample_vec
}

fn takes_and_gives_ownership(mut sample_vec: Vec<i32>) -> Vec<i32> {
    sample_vec.push(5);
    sample_vec
}
