use crate::constants::operators::TERNARY_OPERATOR;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_func_usage::parse_func_usage;
use crate::parsers::parse_object_accessors::parse_object_accessors;
use crate::parsers::parse_ternary_op::parse_ternary_op;
use crate::TokenStream;

pub fn parse_call(stream: &mut TokenStream, node: Node) -> Node {
    let pos = stream.pos();

    if stream.is_punc('(') {
        if !node.value.is_dyn_fn() {
            stream.panic("Cannot call a non function");
            unreachable!()
        }
        let data = node.value.to_dyn_fn();
        let data = data.borrow();
        let data = data.to_dyn_fn();

        let args = parse_delimited(stream, '(', ')', ',', &parse_expression);
        Nodes::create(
            Nodes::DynFnCall {
                params: data.0.clone(),
                args: args.into_iter().map(|n| Box::new(n)).collect(),
                body: data.1.clone(),
            },
            pos
        )
    } else if stream.is_kw("->") {
        return parse_object_accessors(stream, node)
    } else if stream.is_op(TERNARY_OPERATOR) {
        return parse_ternary_op(stream, node)
    } else {
        node
    }
}
