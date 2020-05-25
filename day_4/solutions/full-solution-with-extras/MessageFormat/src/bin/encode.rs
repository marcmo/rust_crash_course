use anyhow::Error;
use msg_format::{ByteView, Message};
use human_panic::setup_panic;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::Iterator;
use structopt::StructOpt;

mod cli;

fn main() -> Result<(), Error> {
    setup_panic!();

    let args = cli::Encoder::from_args();
    let reader: BufReader<Box<dyn std::io::Read>> = args.input.into();
    let mut writer: BufWriter<Box<dyn std::io::Write>> = args.output.into();
    let lines: Box<dyn Iterator<Item = std::io::Result<String>>> = if let Some(count) = args.count {
        Box::new(reader.lines().take(count).into_iter())
    } else {
        Box::new(reader.lines())
    };

    for result in lines {
        match result {
            Ok(line) => {
                let m: Message = serde_json::from_str(&line)?;
                let bytes: Vec<u8> = ByteView::new(m).into_iter().collect();
                writer.write(&bytes)?;
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(-1)
            }
        }
    }
    Ok(())
}
