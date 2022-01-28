use crate::constants::keywords::RETURN_KEYWORD;
use crate::core::nodes::Nodes;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_return(stream: &mut TokenStream) -> Nodes {
    stream.skip_kw(RETURN_KEYWORD);

    Nodes::Return(Box::new(parse_expression(stream)))
}