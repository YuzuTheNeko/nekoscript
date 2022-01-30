use crate::core::nodes::{Node, Nodes};
use crate::core::nodes::Nodes::FnCall;
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_func_usage(
    stream: &mut TokenStream,
    kw: Option<String>,
    func: Option<Box<Node>>,
) -> Node {
    let pos = stream.pos();

    let params = parse_delimited(stream, '(', ')', ',', &parse_expression);

    Nodes::create(
        FnCall {
            name: kw,
            func,
            args: params.into_iter().map(|n| Box::new(n)).collect(),
        },
        pos
    )
}
