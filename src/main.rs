use structopt::StructOpt;
use std::fs::OpenOptions;

use std::fs::{File, remove_file};
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use read_iter::ReadIter; // also add dependency to Cargo.toml
use nop_json::{Reader, Value};


#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    // pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    if Path::new(&args.output).exists() {
        remove_file(&args.output).unwrap();
    }

    let mut out_file = OpenOptions::new()
      .write(true)
      .create_new(true)
      .append(true)
      .open(&args.output)
      .unwrap();

    let mut file = ReadIter::new(File::open(args.path).unwrap());
    let mut reader = Reader::new(&mut file);

    let root: Vec<Value> = reader.read().unwrap();
    for value in root {
        write!(out_file, "{}\n", value);
    }
    /*for value in stream {
        println!("{}", value.unwrap());
    }*/

    /*let de = Deserializer::from_slice(&buffer);
    let mut stream = de.into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }*/
}
