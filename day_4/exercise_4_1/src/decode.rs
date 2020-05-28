
use std::env;
use std::env::args;

fn main() {
    
    println!("----------");
    println!("Decode");
    println!("");
    
    
    let args_vec: Vec<String> = env::args().collect();
    println!("Args {:?}", args_vec);
    
    if args_vec.len() != 2 {
        println!("Wrong argument");
        std::process::exit(1);
    }
}
