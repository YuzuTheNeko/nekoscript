use crate::core::nodes::{Accessor, Nodes};
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;
use crate::util::chars::is_id;

pub fn parse_object_accessors(stream: &mut TokenStream, left: Nodes) -> Nodes {
    let mut accessors = vec![];

    while stream.is_kw("->") {
        stream.skip_kw("->");

        let name = stream.read_while(&is_id);

        if stream.is_punc('(') {
            let got = parse_delimited(stream, '(', ')', ',', &parse_expression);
            accessors.push(Accessor::Method(name, got))
        } else {
            accessors.push(Accessor::Property(name))
        }
    }

    Nodes::ObjectAccessor {
        value: Box::new(left),
        accessors
    }
}