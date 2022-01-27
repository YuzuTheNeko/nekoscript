use crate::core::nodes::Nodes;
use crate::core::nodes::Nodes::FnCall;
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_func_usage(
    stream: &mut TokenStream,
    kw: Option<String>,
    func: Option<Box<Nodes>>,
) -> Nodes {
    let params = parse_delimited(stream, '(', ')', ',', &parse_expression);

    FnCall {
        name: kw,
        func,
        args: params.into_iter().map(|n| Box::new(n)).collect(),
    }
}
