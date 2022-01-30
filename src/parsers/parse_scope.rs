use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_scope(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    let got = parse_delimited(stream, '{', '}', ';', &parse_expression);
    Nodes::create(
        Nodes::Scope(got),
        pos
    )
}
