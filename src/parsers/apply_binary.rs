use crate::core::nodes::{Node, Nodes};
use crate::parsers::parse_atom::parse_atom;
use crate::TokenStream;

pub fn apply_binary(stream: &mut TokenStream, left: Node, prec: u8) -> Node {
    let peek = stream.peek();

    if !peek.value.is_op() {
        return left;
    }

    let op = peek.value.to_op();

    let next_prec = op.prec();

    if next_prec > prec {
        stream.read_next();

        let pos = stream.pos();

        let right = parse_atom(stream);

        let right = apply_binary(stream, right, next_prec);

        let bin = Nodes::BinaryExpr {
            left: Box::new(left),
            right: Box::new(right),
            op: op.clone(),
        };

        apply_binary(stream, Nodes::create(bin, pos), prec)
    } else {
        left
    }
}
