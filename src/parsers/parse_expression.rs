use crate::core::nodes::Nodes;
use crate::parsers::apply_binary::apply_binary;
use crate::parsers::parse_atom::parse_atom;
use crate::parsers::parse_call::parse_call;
use crate::TokenStream;

pub fn parse_expression(stream: &mut TokenStream) -> Nodes {
    let atom = parse_atom(stream);

    let atom = parse_call(stream, atom);

    apply_binary(stream, atom, 0)
}
