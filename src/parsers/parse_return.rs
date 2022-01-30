use crate::constants::keywords::RETURN_KEYWORD;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_return(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    stream.skip_kw(RETURN_KEYWORD);

    Nodes::create(
        Nodes::Return(Box::new(parse_expression(stream))),
        pos
    )
}