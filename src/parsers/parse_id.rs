use crate::core::nodes::{Node, Nodes};
use crate::core::token_stream::TokenStream;
use crate::parsers::parse_func_usage::parse_func_usage;
use crate::util::chars::is_id;

pub fn parse_strict_id(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    Nodes::create(
        Nodes::Keyword(stream.read_while(&is_id)),
        pos
    )
}

pub fn parse_id(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    let got = stream.read_while(&is_id);

    if stream.is_punc('(') {
        return parse_func_usage(stream, Some(got), None);
    }

    Nodes::create(
        Nodes::Keyword(got),
        pos
    )
}
