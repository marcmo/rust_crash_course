
use std::env;

use std::fs::File;
use std::io::Read;

use std::vec;


struct Message {
    version: u8,
    tp: u8,
    reserved: u8,
    id1: u8,
    id2: u32,
    timestamp: u32,
    data: Vec<u8>,
}

impl Message {
    pub fn new() -> Self {
        Message {
            version: 0,
            tp: 0,
            reserved: 0,
            id1: 0,
            id2: 0,
            timestamp: 0,
            data : Vec::new(),
        }
    }
}

struct Decoder {
    state: u8,
    msg : Message,
}

impl Decoder {
    
    pub fn new() -> Self {
        Decoder {
            state : 0,
            msg : Message::new(),
        }
    }
    
    pub fn decode(&mut self, src: &[u8]) {
        
        for c in src.iter() {
            
            dbg!(self.state, c);
            
            match self.state {
                
                0 => {
                    self.msg.version = *c;
                    self.state += 1;
                }
                
                1 => {
                    self.msg.tp = *c;
                    self.state += 1;
                }
                
                2 => {
                    self.msg.reserved = *c;
                    self.state += 1;
                }
                
                3 => {
                    self.msg.id1 = *c;
                    self.state += 1;
                }
                
//                 0 => {
//                     self.msg.version = *c;
//                     self.state += 1;
//                 }
                
                
                _ => {println!("Bad state");}
            }
        }
        
    }
}


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
        Err(_) => {
            println!("Error oppen file {}", file_name);
            std::process::exit(1);
        }
    };
    
    
    
    println!("File opened");
    
    let mut dc = Decoder::new();
    
    let mut buf=[0u8;12];
    
    let rd = file.read(&mut buf);
    match rd {
        Err(_) => {
            println!("Error READING");
        }
        Ok(n) => {
            dc.decode(&buf[0..n]);
        }
    }
    
    
}
