use crate::constants::operators::TERNARY_OPERATOR;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_ternary_op(stream: &mut TokenStream, node: Node) -> Node {
    let pos = stream.pos();

    stream.skip_op(TERNARY_OPERATOR);

    let when_true = parse_expression(stream);

    stream.skip_punc(':');

    let when_false = parse_expression(stream);

    Nodes::create(
        Nodes::Ternary {
            condition: Box::new(node),
            when_true: Box::new(when_true),
            when_false: Box::new(when_false)
        },
        pos
    )
}