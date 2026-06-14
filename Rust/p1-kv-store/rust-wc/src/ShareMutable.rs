use std::rc::Rc;
use std::cell::RefCell;

fn main(){
    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    *a.borrow_mut() +=1;
    *b.borrow_mut() +=10;

    println!("count {}", counter.borrow());
    println!("Owners {}", Rc::strong_count(&counter));

    let m1 = counter.borrow_mut();
    // let m2 = counter.borrow_mut(); // double borrow runtime panic
}