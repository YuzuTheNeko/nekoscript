use crate::constants::operators::{ADD, ASSIGN, DIV, EQUALS, GREATER, GREATER_EQ, LESSER, LESSER_EQ, MULTI, NOT_EQUALS, SUB, SUB_EQUAL, SUM_EQUAL};
use crate::core::nodes::{Node, Nodes};
use crate::core::operator_types::OperatorTypes;
use crate::core::token_stream::TokenStream;
use crate::util::chars::is_op;

pub fn parse_op(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    let op = stream.read_while(&is_op);

    Nodes::create({
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
        } else if op.eq(DIV) {
            Nodes::Operator(OperatorTypes::Div)
        } else if op.eq(MULTI) {
            Nodes::Operator(OperatorTypes::Multi)
        } else if op.eq(GREATER) {
            Nodes::Operator(OperatorTypes::Greater)
        } else if op.eq(GREATER_EQ) {
            Nodes::Operator(OperatorTypes::GreaterEq)
        } else if op.eq(LESSER_EQ) {
            Nodes::Operator(OperatorTypes::LesserEq)
        } else if op.eq(LESSER) {
            Nodes::Operator(OperatorTypes::Lesser)
        } else {
            stream.panic(&format!("Unsupported operator {}", op));
            unreachable!()
        }
    }, pos)
}
