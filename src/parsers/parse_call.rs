use crate::core::nodes::Nodes;
use crate::parsers::parse_func_usage::parse_func_usage;
use crate::TokenStream;

pub fn parse_call(stream: &mut TokenStream, node: Nodes) -> Nodes {
    if stream.is_punc('(') {
        if !node.is_fn() && !node.is_dyn_fn() {
            stream.panic("Cannot call a non function");
            unreachable!()
        }
        unimplemented!()
    } else {
        node
    }
}