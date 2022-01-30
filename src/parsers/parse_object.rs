use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use crate::constants::keywords::OBJECT_KEYWORD;
use crate::core::data_types::DataTypes;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_expression::parse_expression;
use crate::TokenStream;
use crate::util::chars::is_id;

pub fn parse_object(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    stream.skip_kw(OBJECT_KEYWORD);

    stream.skip_punc('{');

    let mut hash: HashMap<String, Node> = HashMap::new();

    while !stream.is_punc('}') {
        let name = stream.read_while(&is_id);

        stream.skip_punc(':');

        let value = parse_expression(stream);

        hash.insert(name, value);

        if !stream.is_punc('}') {
            stream.skip_punc(',')
        }
    }

    stream.skip_punc('}');

    Nodes::create(
        Nodes::Value(Rc::new(RefCell::new(DataTypes::Object(Rc::new(RwLock::new(hash)))))),
        pos
    )
}