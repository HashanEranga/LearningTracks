use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub(crate) fn multi_worker(){
    let (tx, rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    for job in 1..10{
        tx.send(job).unwrap();
    }
    drop(tx);

    let rx = Arc::new(Mutex::new(rx));
    // let mut handles = vec![];

    for id in 0..3 {
        let rx = Arc::clone(&rx);
        let result_tx = result_tx.clone();
        thread::spawn(move || {
            loop{
                let job = rx.lock().unwrap().recv();  
                    match job {
                        Ok(job) => {
                            let result = job * 2;
                            result_tx.send((id, job, result)).unwrap();
                            thread::sleep(std::time::Duration::from_millis(200));
                        },
                        Err(_)  => break, 
                    }
            }
        });
        // handles.push(handle);
    }

    drop(result_tx);

    // for h in handles {
    //     h.join().unwrap();
    // }

    for (id, job, result) in result_rx{
        println!("worker {id}: job {job} -> {result}");
    }
}