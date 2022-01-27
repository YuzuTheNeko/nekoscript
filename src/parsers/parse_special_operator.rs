use crate::constants::operators::{ASSIGN, OPERATORS, SUB_EQUAL, SUM_EQUAL};
use crate::core::nodes::Nodes;
use crate::core::operator_types::SpecialOperatorTypes;
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_op::parse_op;
use crate::util::chars::is_op;
use crate::TokenStream;

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
        value: Box::new(expr),
    }
}
