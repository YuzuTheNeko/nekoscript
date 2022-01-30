use crate::constants::keywords::{FALSE, TRUE};
use crate::core::data_types::DataTypes;
use crate::core::nodes::{Node, Nodes};
use crate::TokenStream;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_bool(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    if stream.is_kw(TRUE) {
        stream.skip_kw(TRUE);
        Nodes::create(
            Nodes::Value(Rc::new(RefCell::new(DataTypes::Bool(true)))),
            pos
        )
    } else {
        stream.skip_kw(FALSE);
        Nodes::create(
            Nodes::Value(Rc::new(RefCell::new(DataTypes::Bool(false)))),
            pos
        )
    }
}
