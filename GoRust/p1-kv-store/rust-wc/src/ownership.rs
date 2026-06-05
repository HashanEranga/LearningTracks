fn main() {
    // println!("Hello, world!");

    let s = String::from("Hashan");
    // let t = s;
    // println!("{s}");
    
    // ownership 
    // in rust we cannot use the variable after it was moved -ownership error
    // why - this is rust way of guranteeing memory saggety without a GC
    
    // fix 1 - clone the value - make an explocit copy
    println!("Fix 1 - clone the value");
    let cloned_s = s.clone();
    
    println!("{} {}", s, cloned_s);
    
    // fix 2 use a reference - borrow instead move
    println!("Fix 2 - use a reference");
    let t = &s;
    println!("{} - {}", s, t);
    
    // fix 3 Derive copy - for simple types with no heap data, opt out of move semantics entirely
    println!("Fix 3 - Derive a copy");
    #[derive(Copy, Clone)]
    struct Point { x: i32, y: i32 }

    let p1 = Point { x: 1, y: 2};
    let p2 = p1; 

    println!("Point 1 X value : {}, Y value : {}", p1.x, p1.y);
    println!("Point 2 X value : {}, Y value : {}", p2.x, p2.y);

}
