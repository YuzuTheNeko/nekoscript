use crate::constants::keywords::{ELSE_IF_KEYWORD, ELSE_KEYWORD, IF_KEYWORD};
use crate::core::nodes::Nodes;
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_scope::parse_scope;
use crate::TokenStream;

pub fn parse_if(stream: &mut TokenStream) -> Nodes {
    stream.skip_kw(IF_KEYWORD);

    let condition = parse_expression(stream);

    let scope = parse_scope(stream);

    let mut els: Option<Box<Nodes>> = None;

    let mut races = vec![];

    while stream.is_kw(ELSE_KEYWORD) {
        if stream.is_kw(ELSE_IF_KEYWORD) {
            stream.skip_kw(ELSE_IF_KEYWORD);

            let condition = parse_expression(stream);

            let scope = parse_scope(stream);

            races.push((condition, scope))
        } else {
            stream.skip_kw(ELSE_KEYWORD);

            els = Some(Box::new(parse_scope(stream)))
        }
    }

    Nodes::If {
        condition: Box::new(condition),
        races,
        when_true: Box::new(scope),
        when_false: els
    }
}