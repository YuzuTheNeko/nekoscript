#![allow(dead_code)]
#![allow(unused)]

use crate::core::interpreter::Interpreter;
use crate::core::token_stream::TokenStream;
use std::fs::{canonicalize, File};
use std::io::Read;
use std::path::Path;
use std::process::exit;

mod constants;
mod core;
mod native;
mod parsers;
mod runtime;
mod tests;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!("Error: no input file given.");
        exit(0)
    }

    let pth = canonicalize(args.get(0).unwrap());

    if pth.is_err() {
        let err = pth.unwrap_err();
        println!("{}", err);
        exit(0)
    }

    let p = pth.unwrap();

    let path = Path::new(&p);

    let mut str = String::new();

    File::open(&path).unwrap().read_to_string(&mut str);

    let mut stream = TokenStream::new(&str, path.to_str().unwrap().to_owned());

    stream.start();

    let itr = Interpreter::new(stream.nodes, path.to_str().unwrap().to_string());

    itr.run(None);
}
