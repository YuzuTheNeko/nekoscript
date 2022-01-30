use crate::constants::operators::{ADD, ASSIGN, EQUALS, MULTI, NOT_EQUALS, SUB, SUB_EQUAL, SUM_EQUAL};
use crate::core::nodes::Nodes;
use crate::core::operator_types::OperatorTypes;
use crate::core::token_stream::TokenStream;
use crate::util::chars::is_op;

pub fn parse_op(stream: &mut TokenStream) -> Nodes {
    let op = stream.read_while(&is_op);

    if op.eq(SUB) {
        Nodes::Operator(OperatorTypes::Sub)
    } else if op.eq(ADD) {
        Nodes::Operator(OperatorTypes::Add)
    } else if op.eq(EQUALS) {
        Nodes::Operator(OperatorTypes::Equals)
    } else if op.eq(NOT_EQUALS) {
        Nodes::Operator(OperatorTypes::NotEquals)
    } else if op.eq(ASSIGN) {
        Nodes::Operator(OperatorTypes::Assign)
    } else if op.eq(SUM_EQUAL) {
        Nodes::Operator(OperatorTypes::AddAssign)
    } else if op.eq(SUB_EQUAL) {
        Nodes::Operator(OperatorTypes::SubAssign)
    } else {
        stream.panic(&format!("Unsupported operator {}", op));
        unreachable!()
    }
}
