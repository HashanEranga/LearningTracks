use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    
    match args.get(1){
        Some(path) => { 
            let contents = std::fs::read_to_string(path)?; 
            println!("Lines : {}", contents.lines().count());
            println!("Words : {}", contents.split_whitespace().count());
            println!("Chars : {}", contents.chars().count());
        }
        None => { println!("Usage: rust-wc <file>")}
    }
    Ok(())
}