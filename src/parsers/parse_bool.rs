use crate::constants::keywords::TRUE;
use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::TokenStream;

pub fn parse_bool(stream: &mut TokenStream) -> Nodes {
    if stream.is_kw(TRUE) {
        Nodes::Value(DataTypes::Bool(true))
    } else {
        Nodes::Value(DataTypes::Bool(false))
    }
}