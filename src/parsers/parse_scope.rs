use crate::core::nodes::Nodes;
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_scope(stream: &mut TokenStream) -> Nodes {
    let got = parse_delimited(stream, '{', '}', ';', &parse_expression);
    Nodes::Scope(got.into_iter().map(| n | Box::new(n)).collect())
}