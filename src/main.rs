#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::core::interpreter::Interpreter;
use crate::core::token_stream::TokenStream;

mod core;
mod constants;
mod tests;
mod util;
mod native;
mod parsers;
mod runtime;

fn main() {
    let path = Path::new("C:\\Users\\user\\Desktop\\projects\\nekoscript\\tests\\example.nksc");

    let mut str = String::new();

    File::open(&path).unwrap().read_to_string(&mut str);

    let mut stream = TokenStream::new(&str);

    stream.start();

    println!("Parsed {} nodes", stream.nodes.len());

    let itr = Interpreter::new(stream.nodes);

    itr.run();
}