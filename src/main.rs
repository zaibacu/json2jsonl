use structopt::StructOpt;
use std::fs::OpenOptions;

use std::fs::{File, remove_file};
use std::io::Write;
use std::io::ErrorKind;
use std::path::Path;
use read_iter::ReadIter; // also add dependency to Cargo.toml
use nop_json::{Reader, Value};

use std::io::BufRead;
use std::io::BufReader;
use std::str;


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
    let mut data_iter = file.skip(1);
    let mut reader = Reader::new(&mut data_iter);

    loop {
        match reader.read::<Value>() {
            Ok(value) => {
                write!(out_file, "{}\n", value);
            },
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(ref e) if e.kind() == ErrorKind::InvalidInput => break,
            Err(ref e) if e.kind() == ErrorKind::Other => {
                if e.to_string() == "$: Invalid JSON: unexpected end of input"{
                    break
                }
                else if e.to_string() == "$: Invalid JSON input: unexpected ','"{
                    continue
                }
                else if e.to_string() == "$: Invalid JSON input: unexpected ']'"{
                    continue
                }
                else {
                    println!("{}", e.to_string());
                    continue
                }
                
            },
            Err(e) => eprintln!("Error: {}, kind: {:?}", e, e.kind()),
        }
    }
    /*
    let root: Vec<Value> = reader.read().unwrap();
    for value in root {
        write!(out_file, "{}\n", value);
    }*/
    /*for value in stream {
        println!("{}", value.unwrap());
    }*/

    /*let de = Deserializer::from_slice(&buffer);
    let mut stream = de.into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }*/
}
