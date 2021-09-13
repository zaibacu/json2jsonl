# About

This is a tool to take large JSON blob and convert it to JSONL file which can be readed as stream

## Build

Rust language and Cargo is used to build project. More info: [Cargo](https://doc.rust-lang.org/cargo/index.html)

## Usage

`json2jsonl <input_file>.json <output_file>.jsonl`

# Caveats

At the moment it expects root element of JSON to be array and each array item as a line in JSONL.

**TODO**: somekind of pattern to control this
