use std::sync::{Arc, Mutex};
use std::thread;
mod test_drop;
mod multi_worker;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let clone = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        let mut guard = clone.lock().unwrap();
        *guard += 10;

        // double-lock same thread deadlock 
        // also Rc RefCell accross threads -> compile errors
        // let mut guard2 = clone.lock().unwrap();
        // *guard2 += 10;
    });

    handle.join().unwrap();
    println!("count {}", counter.lock().unwrap());

    test_drop::test_drop();
    multi_worker::multi_worker();
}
