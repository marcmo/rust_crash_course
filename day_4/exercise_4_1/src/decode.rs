
use std::env;

use std::fs::File;
use std::io::Read;

use std::vec;


#[derive(Debug)]
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


struct Decoder4b {
    data : u32,
    state : u8,
}

impl Decoder4b {
    pub fn new() -> Self {
        Decoder4b {
            state: 0,
            data: 0,
        }
    }
    
    pub fn decode (&mut self, input : u8) -> Option<u32> {
        if self.state < 4 {
            self.data = self.data << 8;
            self.data |= input as u32;
            self.state += 1;
        }
        
        dbg!(self.data);
        
        if self.state >= 4 {
            
            let res = self.data;
            
            self.data = 0;
            self.state = 0;
            
            Some(res)
        } else {
            None
        }
    }
}

struct Decoder {
    state: u8,
    keep : Decoder4b,
    data_size: u32,
    msg : Message,
}

impl Decoder {
    
    pub fn new() -> Self {
        Decoder {
            state : 0,
            keep : Decoder4b::new(),
            data_size: 0,
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
                
                4 => {
                    let ret = self.keep.decode(*c);
                    match ret {
                        Some(num) => {
                            self.msg.id2 = num;
                            self.state += 1;
                        }
                        None => {}
                    }
                }
                
                5 => {
                    let ret = self.keep.decode(*c);
                    match ret {
                        Some(num) => {
                            self.msg.timestamp = num;
                            self.state += 1;
                        }
                        None => {}
                    }
                }
                
                
                6 => {
                    let ret = self.keep.decode(*c);
                    match ret {
                        Some(num) => {
                            self.data_size = num;
                            self.state += 1;
                        }
                        None => {}
                    }
                }
                
                7 => {
                    dbg!(&self.data_size);
                    
                    self.msg.data.push(*c);
                    
                    if self.msg.data.len() >= self.data_size as usize {
                        println!("{:?}", &self.msg);
                    
                    
                    self.msg = Message::new();
                    self.data_size = 0;
                    self.state = 0;
                    }
                    
                }
                
                
                _ => {eprintln!("Bad state");}
            }
        }
        
    }
}


fn main() {
    
    eprintln!("----------");
    eprintln!("Decode");
    eprintln!("");
    
    
    let args_vec: Vec<String> = env::args().collect();
    eprintln!("Args {:?}", args_vec);
    
    if args_vec.len() != 2 {
        eprintln!("Wrong argument");
        std::process::exit(1);
    }
    
    let file_name = &args_vec[1];
    
    eprintln!("Read from {:?}", file_name);
    
    
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("Error oppen file {}", file_name);
            std::process::exit(1);
        }
    };
    
    
    
    eprintln!("File opened");
    
    let mut dc = Decoder::new();
    
    let mut buf=[0u8;256];
    
    let rd = file.read(&mut buf);
    match rd {
        Err(_) => {
            eprintln!("Error READING");
        }
        Ok(n) => {
            dc.decode(&buf[0..n]);
        }
    }
    
    
}
