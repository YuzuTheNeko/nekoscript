use crate::constants::keywords::FUNCTION_DEFINITION;
use crate::core::nodes::Nodes;
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_id::parse_strict_id;
use crate::parsers::parse_scope::parse_scope;
use crate::TokenStream;
use crate::util::chars::is_id;

pub fn parse_func_def(stream: &mut TokenStream) -> Nodes {
    stream.skip_kw(FUNCTION_DEFINITION);

    stream.skip_useless();

    let name = stream.read_while(&is_id);

    let params = parse_delimited(stream, '(', ')', ',', &parse_strict_id);

    let scope = parse_scope(stream);

    Nodes::FnDef {
        name,
        params: params.into_iter().map(| n | n.to_kw().clone()).collect(),
        body: Box::new(scope)
    }
}