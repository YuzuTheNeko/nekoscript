use crate::constants::keywords::VARIABLE_KEYWORD;
use crate::constants::operators::ASSIGN;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_variable(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    stream.skip_kw(VARIABLE_KEYWORD);

    let name = stream.read_next();

    let name = name.value.to_kw();

    stream.skip_kw(ASSIGN);

    let value = parse_expression(stream);

    Nodes::create(
        Nodes::VariableDef {
            name: name.clone(),
            value: Box::new(value),
        },
        pos
    )
}
