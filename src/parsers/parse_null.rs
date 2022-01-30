use crate::constants::keywords::NULL;
use crate::core::data_types::DataTypes;
use crate::core::nodes::{Node, Nodes};
use crate::TokenStream;

pub fn parse_null(stream: &mut TokenStream) -> Node {
    let pos = stream.pos();

    stream.skip_kw(NULL);

    Nodes::create(
        Nodes::Value(DataTypes::null()),
        pos
    )
}