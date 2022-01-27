use crate::core::nodes::Nodes;
use crate::util::chars::is_punc;
use crate::TokenStream;

pub fn parse_punc(stream: &mut TokenStream) -> Nodes {
    Nodes::Punc(stream.read_while(&is_punc))
}
