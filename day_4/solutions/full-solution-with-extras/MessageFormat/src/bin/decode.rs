use anyhow::Error;
use msg_format::{Message, MessageIterator};
use human_panic::setup_panic;
use std::io::BufReader;
use std::iter::Iterator;
use structopt::StructOpt;

mod cli;

fn main() -> Result<(), Error> {
    setup_panic!();

    let args = cli::Decoder::from_args();
    let reader: BufReader<Box<dyn std::io::Read>> = args.input.into();
    let messages: Box<dyn Iterator<Item = std::io::Result<Message>>> =
        if let Some(count) = args.count {
            Box::new(MessageIterator::new(reader).take(count).into_iter())
        } else {
            Box::new(MessageIterator::new(reader))
        };

    for result in messages {
        match result {
            Ok(msg) => {
                let s = serde_json::to_string(&msg)?;
                println!("{}", s);
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(-1)
            }
        };
    }
    Ok(())
}
