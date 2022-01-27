use crate::core::nodes::Nodes;
use crate::TokenStream;

pub fn parse_delimited<'a>(
    stream: &mut TokenStream,
    start: char,
    stop: char,
    sep: char,
    f: &'a dyn Fn(&mut TokenStream) -> Nodes,
) -> Vec<Nodes> {
    let mut vc = vec![];

    stream.skip_punc(start);

    let mut first = true;
    let mut ended = false;

    while !stream.eof() {
        if first {
            first = false
        } else {
            stream.skip_punc(sep)
        }

        if stream.is_punc(stop) {
            ended = true;
            break;
        }

        vc.push(f(stream));

        if stream.is_punc(stop) {
            ended = true;
            break;
        }
    }

    stream.skip_punc(stop);

    vc
}
