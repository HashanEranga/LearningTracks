use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub(crate) fn multi_worker(){
    let (tx, rx) = mpsc::channel();

    for job in 1..9{
        tx.send(job).unwrap();
    }
    drop(tx);

    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];

    for id in 0..3 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop{
                let job = rx.lock().unwrap().recv();  
                    match job {
                        Ok(job) => println!("worker {id} got job {job}"),
                        Err(_)  => break, 
                    }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}