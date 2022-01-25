use std::cell::RefCell;
use std::rc::Rc;
use crate::constants::operators::SUB;
use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::core::token_stream::TokenStream;
use crate::util::chars::is_digit;

pub fn parse_number(stream: &mut TokenStream) -> Nodes {
    let is_negative = {
        if stream.is_op(SUB) {
            stream.skip_op(SUB);
            true
        } else {
            false
        }
    };

    let int = stream.read_while(&is_digit);

    if let Ok(int) = int.parse::<i64>() {
        Nodes::Value(
            Rc::new(RefCell::new(DataTypes::Int(int)))
        )
    } else {
        stream.panic(&format!("{} is not a valid integer", int));
        unreachable!()
    }
}