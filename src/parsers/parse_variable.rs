use crate::constants::keywords::VARIABLE_KEYWORD;
use crate::constants::operators::ASSIGN;
use crate::core::nodes::Nodes;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_variable(stream: &mut TokenStream) -> Nodes {
    stream.skip_kw(VARIABLE_KEYWORD);

    let name = stream.read_next();

    let name = name.to_kw();

    stream.skip_kw(ASSIGN);

    let value = parse_expression(stream);

    Nodes::VariableDef {
        name: name.clone(),
        value: Box::new(value),
    }
}
