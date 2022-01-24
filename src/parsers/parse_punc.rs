use crate::core::nodes::Nodes;
use crate::TokenStream;
use crate::util::chars::is_punc;

pub fn parse_punc(stream: &mut TokenStream) -> Nodes {
    Nodes::Punc(stream.read_while(&is_punc))
}