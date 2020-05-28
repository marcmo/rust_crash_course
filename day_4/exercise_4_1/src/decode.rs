
use std::env;

use std::fs::File;
use std::io::Read;

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
    
    let file_name = &args_vec[1];
    
    println!("Read from {:?}", file_name);
    
    
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("Error oppen file {}", file_name);
            std::process::exit(1);
        }
    };
    
    
    
    println!("File opened");
    
    
    let mut buf=[0u8;12];
    
    file.read(&mut buf).unwrap();
}
