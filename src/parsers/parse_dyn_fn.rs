use crate::constants::keywords::DYN_FN;
use crate::core::data_types::DataTypes;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_delimited::parse_delimited;
use crate::parsers::parse_id::parse_id;
use crate::parsers::parse_scope::parse_scope;
use crate::TokenStream;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_dyn_fn(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    stream.skip_kw(DYN_FN);

    let got = parse_delimited(stream, '(', ')', ',', &parse_id);

    let scope = parse_scope(stream);

    Nodes::create(
        Nodes::Value(Rc::new(RefCell::new(DataTypes::Fn {
            params: got.iter().map(|n| n.value.to_kw().clone()).collect(),
            body: Box::new(scope),
        }))),
        pos
    )
}
