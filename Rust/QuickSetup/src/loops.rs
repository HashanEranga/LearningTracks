pub(crate) fn loops() {
    'outer: loop {
        loop {
            println!("loop");
            break 'outer;
        }
    }

    let counter = loop {
        break 10;
    };

    println!("The result is {}", counter);

    let vector = vec![1, 2, 3, 4, 5];
    for i in vector {
        println!("{}", i);
    }

    let mut num = 0;

    while num < 10 {
        println!("{}", num);
        num += 1;
    }
}
