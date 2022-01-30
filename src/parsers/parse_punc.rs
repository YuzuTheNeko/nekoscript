use crate::core::nodes::{Node, Nodes};
use crate::util::chars::is_punc;
use crate::TokenStream;

pub fn parse_punc(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();
    Nodes::create(
        Nodes::Punc(stream.read_while(&is_punc)),
        pos
    )
}
