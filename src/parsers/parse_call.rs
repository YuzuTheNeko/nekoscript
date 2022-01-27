use crate::core::nodes::Nodes;
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_func_usage::parse_func_usage;
use crate::TokenStream;

pub fn parse_call(stream: &mut TokenStream, node: Nodes) -> Nodes {
    if stream.is_punc('(') {
        if !node.is_dyn_fn() {
            stream.panic("Cannot call a non function");
            unreachable!()
        }
        let data = node.to_dyn_fn();
        let data = data.borrow();
        let data = data.to_dyn_fn();

        let args = parse_delimited(stream, '(', ')', ',', &parse_expression);
        Nodes::DynFnCall {
            params: data.0.clone(),
            args: args.into_iter().map(|n| Box::new(n)).collect(),
            body: data.1.clone(),
        }
    } else {
        node
    }
}
