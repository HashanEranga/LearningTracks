use std::sync::mpsc;

pub(crate) fn test_drop(){
    println!("test drop");
    let (tx, rx) = mpsc::channel();

    tx.send(100).unwrap();
    tx.send(200).unwrap();
    drop(tx);

    for job in rx {
        println!("Job : {:?}", job);
    }
}