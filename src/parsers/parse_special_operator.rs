use crate::constants::operators::{ASSIGN, SUB_EQUAL, SUM_EQUAL};
use crate::core::nodes::Nodes;
use crate::core::operator_types::SpecialOperatorTypes;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;
use crate::util::chars::is_op;

pub fn parse_special_op(stream: &mut TokenStream, kw: String) -> Nodes {
    let op = stream.read_while(&is_op);

    let expr = parse_expression(stream);

    Nodes::SpecialAssignment {
        op: {
            if op.eq(SUB_EQUAL) {
                SpecialOperatorTypes::SubAssign
            } else if op.eq(SUM_EQUAL) {
                SpecialOperatorTypes::AddAssign
            } else if op.eq(ASSIGN) {
                SpecialOperatorTypes::Assign
            } else {
                stream.panic(&format!("Unexpected token {}", op));
                unreachable!()
            }
        },
        keyword: kw,
        value: Box::new(expr)
    }
}