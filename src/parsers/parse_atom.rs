use crate::constants::keywords::{DYN_FN, FALSE, FUNCTION_DEFINITION, IF_KEYWORD, NULL, OBJECT_KEYWORD, RETURN_KEYWORD, TRUE, VARIABLE_KEYWORD, WHILE_KEYWORD};
use crate::core::data_types::DataTypes;
use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_bool::parse_bool;
use crate::parsers::parse_dyn_fn::parse_dyn_fn;
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_function_definition::parse_func_def;
use crate::parsers::parse_if::parse_if;
use crate::parsers::parse_null::parse_null;
use crate::parsers::parse_object::parse_object;
use crate::parsers::parse_return::parse_return;
use crate::parsers::parse_scope::parse_scope;
use crate::parsers::parse_variable::parse_variable;
use crate::parsers::parse_while::parse_while;
use crate::TokenStream;

pub fn parse_atom(stream: &mut TokenStream) -> Node {
    if stream.is_punc('(') {
        stream.skip_punc('(');
        let expr = parse_expression(stream);
        stream.skip_punc(')');
        return expr;
    }

    if stream.is_kw(FALSE) || stream.is_kw(TRUE) {
        return parse_bool(stream);
    } else if stream.is_kw(NULL) {
        return parse_null(stream)
    } else if stream.is_kw(VARIABLE_KEYWORD) {
        return parse_variable(stream);
    } else if stream.is_punc('{') {
        return parse_scope(stream);
    } else if stream.is_kw(DYN_FN) {
        return parse_dyn_fn(stream);
    } else if stream.is_kw(IF_KEYWORD) {
        return parse_if(stream);
    } else if stream.is_kw(WHILE_KEYWORD) {
        return parse_while(stream);
    } else if stream.is_kw(FUNCTION_DEFINITION) {
        return parse_func_def(stream);
    } else if stream.is_kw(OBJECT_KEYWORD) {
        return parse_object(stream)
    } else if stream.is_kw(RETURN_KEYWORD) {
        return parse_return(stream)
    }

    stream.read_next()
}
