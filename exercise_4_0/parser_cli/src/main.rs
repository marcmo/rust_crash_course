use std::env;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead};
use bytebuffer::{ByteBuffer};
use msgparser::{ MsgBuffer};

fn parse_file(file_name: PathBuf) -> Result<(), std::io::Error> {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            // let mut msgs: Vec<Msg> = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_) => {
                    let mut msg_buffer = MsgBuffer::new(ByteBuffer::from_bytes(&buffer));
                    loop {
                        match msg_buffer.next() {
                            Some(msg) => println!("{}", msg),
                            None => { break }
                        }
                    }
                    Ok(())
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() == 0 {
        // Read from stdin (pipe case)
        // example: ls | grep ".bin"  | ./parser_cli
        let stdin = io::stdin();
        let mut errors: Vec<String> = vec!();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    let path = Path::new(&line);
                    match parse_file(path.to_path_buf()) {
                        Ok(()) => (),
                        Err(e) => {
                            errors.push(format!("{}", e));
                        },
                    }
                },
                Err(e) => {
                    errors.push(format!("{}", e));
                }
            }
        }
        if errors.len() != 0 {
            println!("\nOops, but some error(s) during parsing:");
            for err in errors {
                println!("\t - {}", err);
            }
        }
    } else {
        // Just parse single file
        let path = Path::new(&args[1]);
        match parse_file(path.to_path_buf()) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
    }

}