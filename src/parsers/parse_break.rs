use crate::constants::keywords::BREAK_LOOP;
use crate::core::nodes::{Node, Nodes};
use crate::TokenStream;

pub fn parse_break(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    stream.skip_kw(BREAK_LOOP);
    Nodes::create(
        Nodes::Break,
        pos 
    )
}