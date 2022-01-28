use crate::constants::keywords::{FALSE, TRUE};
use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::TokenStream;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_bool(stream: &mut TokenStream) -> Nodes {
    if stream.is_kw(TRUE) {
        stream.skip_kw(TRUE);
        Nodes::Value(Rc::new(RefCell::new(DataTypes::Bool(true))))
    } else {
        stream.skip_kw(FALSE);
        Nodes::Value(Rc::new(RefCell::new(DataTypes::Bool(false))))
    }
}
