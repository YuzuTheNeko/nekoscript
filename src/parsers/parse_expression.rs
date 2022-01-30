use crate::core::nodes::{Node, Nodes};
use crate::parsers::apply_binary::apply_binary;
use crate::parsers::parse_atom::parse_atom;
use crate::parsers::parse_call::parse_call;
use crate::TokenStream;

pub fn parse_expression(stream: &mut TokenStream) -> Node {
    let atom = parse_atom(stream);

    let atom = parse_call(stream, atom);

    let atom = apply_binary(stream, atom, 0);

    parse_call(stream, atom)
}
