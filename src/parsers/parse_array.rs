use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;

pub fn parse_array(stream: &mut TokenStream) -> Nodes {
    stream.skip_punc('[');

    let mut vc = vec![];

    while !stream.is_punc(']') {
        vc.push(parse_expression(stream));
        if !stream.is_punc(']') {
            stream.skip_punc(',')
        }
    }

    stream.skip_punc(']');

    Nodes::Value(Rc::new(RefCell::new(DataTypes::Array(Rc::new(RwLock::new(vc))))))
}