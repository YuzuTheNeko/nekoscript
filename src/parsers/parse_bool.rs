use std::cell::RefCell;
use std::rc::Rc;
use crate::constants::keywords::TRUE;
use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::TokenStream;

pub fn parse_bool(stream: &mut TokenStream) -> Nodes {
    if stream.is_kw(TRUE) {
        Nodes::Value(
            Rc::new(RefCell::new(DataTypes::Bool(true)))
        )
    } else {
        Nodes::Value(
            Rc::new(RefCell::new(DataTypes::Bool(false)))
        )
    }
}