pub(crate) fn borrowing() {
    // borrowing rules
    // at any time, there can be only one mutable reference or multiple immutable references to a value
    // references must be valid at all times

    let sample_vec = vec![1, 2, 3, 4, 5];

    let ref1 = &sample_vec;
    let ref2 = &sample_vec;

    let mut sample_vec_two = vec![3, 4, 5, 6, 7];
    let ref3 = &mut sample_vec_two;
    // let ref4 = &mut sample_vec_two;

    println!("{:?}, {:?}, {:?}", ref1, ref2, ref3);
}
