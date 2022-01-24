use crate::constants::keywords::{FALSE, TRUE, VARIABLE_KEYWORD};
use crate::core::nodes::Nodes;
use crate::parsers::parse_bool::parse_bool;
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_scope::parse_scope;
use crate::parsers::parse_variable::parse_variable;
use crate::TokenStream;

pub fn parse_atom(stream: &mut TokenStream) -> Nodes {
    if stream.is_punc('(') {
        stream.skip_punc('(');
        let expr = parse_expression(stream);
        stream.skip_punc(')');
        return expr;
    }

    if stream.is_kw(FALSE) || stream.is_kw(TRUE) {
        return parse_bool(stream)
    } else if stream.is_kw(VARIABLE_KEYWORD) {
        return parse_variable(stream)
    } else if stream.is_punc('{') {
        return parse_scope(stream)
    }

    let got = stream.read_next();

    match got {
        Nodes::Keyword(str) => {
            Nodes::Keyword(str)
        },
        _ => got
    }
}