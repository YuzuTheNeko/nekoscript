use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::core::token_stream::TokenStream;
use crate::util::chars::is_quote;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_string(stream: &mut TokenStream) -> Nodes {
    stream.skip_punc('"');

    let str = stream.read_while(&|u| !is_quote(u));

    if stream.eof() {
        stream.panic("Unexpected end of string");
        unreachable!()
    }

    stream.skip_punc('"');

    Nodes::Value(Rc::new(RefCell::new(DataTypes::Text(str))))
}
