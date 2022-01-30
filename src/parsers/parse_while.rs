use crate::constants::keywords::WHILE_KEYWORD;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_scope::parse_scope;
use crate::TokenStream;

pub fn parse_while(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    stream.skip_kw(WHILE_KEYWORD);

    let condition = parse_expression(stream);

    let scope = parse_scope(stream);

    Nodes::create(
        Nodes::While {
            condition: Box::new(condition),
            scope: Box::new(scope),
        },
        pos
    )
}
